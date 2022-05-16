import { BillingRepository } from "./../billing/billing.repository";
import { BillingService } from "./../billing/billing.service";
import { Injectable } from "@nestjs/common";
import { CreateRegressDto } from "./dto/create-regress.dto";
import { Billing } from "../../entities/Billing/billing.entity";
import Log from "../../shared/providers/log-gcb";
import { InjectRepository } from "@nestjs/typeorm";
import { IBillingRepository } from "../billing/interface";
import { dateDifferenceInDays } from "../../shared/utils/date.util";
import { Cron, CronExpression } from "@nestjs/schedule";
import { addMonths, addDays } from "date-fns";
import logGcb from "../../shared/providers/log-gcb";
import { RulerRepository } from "../ruler/ruler.repository";
import { WebhookService } from "../webhook/webhook.service";

@Injectable()
export class RegressService {
  constructor(
    private readonly billingRepository: IBillingRepository,
    private readonly billingService: BillingService,
    private rulerRepository: RulerRepository,
    private readonly webhookService: WebhookService
  ) {}

  create(createRegressDto: CreateRegressDto) {
    return "This action adds a new regress";
  }

  async findAll() {
    return await this.billingRepository.findAllExpired();
  }

  async find(date: Date) {
    return await this.billingRepository.findExpired(date);
  }

  // update(id: number, updateRegressDto: UpdateRegressDto) {
  //   return `This action updates a #${id} regress`;
  // }

  // remove(id: number) {
  //   return `This action removes a #${id} regress`;
  // }
  async findAllExpired() {
    return await this.billingRepository.findAllExpired();
  }

  async findExpired(date: Date) {
    return await this.billingRepository.findExpired(date);
  }

  async processExpired() {
    let billings: Billing[] = [];

    billings = await this.findAllExpired();

    const billingToRegress = billings.filter((billing) => {
      return (
        dateDifferenceInDays(new Date(), billing.dueDate) >= 3 &&
        billing.regress?.enable &&
        !billing.originId &&
        !billing.regress.reference
      );
    });

    for await (const billing of billingToRegress) {
      /* const regressRuler = await this.rulerRepository.findById(
        billing.ruler.regress_ruler_reference,
      ); */

      try {
        const newBilling = new Billing();
        Object.assign(newBilling, {
          originId: billing.id,
          dueDate: addDays(new Date(), 4).toISOString(),
          ruler: /* regressRuler ? regressRuler : */ billing.ruler,
          negativation: billing?.regress?.negativation,
          value: {
            rate: billing.value?.updatedRate
              ? billing.value?.updatedRate
              : billing.value?.rate,
          },
          statusPayment: "pendent",
          payer: {
            ...billing.issuer,
            address: {
              city: "São Paulo",
              district: "Cidade Monções",
              number: "206",
              state: "SP",
              street: "Rua George Ohm",
              zipCode: "04576-020",
            },
          },
          issuer: {
            name: "FMI SECURITIZADORA S/A",
            document: {
              type: "cnpj",
              number: "20541441000108",
            },
            email: "fmi@gcbinvestimentos.com",
            phone: {},
            address: {
              street: "123",
              number: "123",
              complement: "123",
              zipCode: "06700-254",
              district: "123",
              city: "123",
              state: "SP",
            },
          },
          paymentMethods: {
            billet: {
              enable: true,
              created: false,
              observation: billing.ruler?.paymentMethods?.billet?.observation,
              grafeno: billing.ruler?.paymentMethods?.billet?.grafeno,
            },
            noticeDeposit: {
              enable: true,
              payout: false,
              grafeno: billing.ruler?.paymentMethods?.noticeDeposit?.grafeno,
            },
          },
          regress: { enable: false, negativation: { enable: false } },
          interest: billing.interest,
          documentNumber: billing.documentNumber,
        });
        /* newBilling.paymentMethods.billet.observation =
          billing.paymentMethods?.billet?.regressObservation ??
          billing.paymentMethods?.billet?.observation; */

        const billingWithId = await this.billingService.create({
          ...newBilling,
          rulerId: billing.ruler.id,
        });

        billing.statusPayment = "regress";
        billing.regress.enable = false;
        billing.regress.reference = billingWithId.id;

        await this.billingRepository.updateBilling({
          id: billing.id,
          data: billing,
        });

        await this.webhookService.paymentUpdate([billing]);
      } catch (error) {
        logGcb.erro("Error on regress", { error });
        continue;
      }
    }
    Log.cron("Regressos processados", JSON.stringify({ ...billingToRegress }));
    return;
  }
}
