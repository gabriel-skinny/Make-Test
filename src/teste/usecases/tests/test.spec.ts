import { IBillingRepository } from "../billing/interface";
import { BillingService } from "./../billing/billing.service";
import { RulerRepository } from "../ruler/ruler.repository";
import { RegressService } from 'src/teste/usecases/regress.service.ts
'

describe('sut_name'), () => {

  IBillingRepositorySpy implements IBillingRepository {}

  BillingServiceSpy implements BillingService {}

  RulerRepositorySpy implements RulerRepository {}



  billingRepository: IBillingRepository
  billingService: BillingService
  rulerRepository: RulerRepository
  sut: RegressService


  beforeEach(() => {)
    billingRepository = new IBillingRepository()
    billingService = new BillingService()
    rulerRepository = new RulerRepository()


    sut = new RegressService(
      billingRepository,
      billingService,
      rulerRepository,
    )
  })   
}
