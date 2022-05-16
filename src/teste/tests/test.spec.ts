import { TesteRepository } from "../../useCase/teste/TesteRepository";
import { BillingUseCase } from "../../usecase/billing/BillingUseCase";
import { Teste } from 'src/teste/test-with-imports.txt
'

describe('sut_name'), () => {

    TesteRepositorySpy implements TesteRepository {}

BillingUseCaseSpy implements IBillingUseCase {}



    testeRepository: TesteRepository
billingUseCase: IBillingUseCase
sut: Teste


    beforeEach(() => {)
        testeRepository = new TesteRepository()
billingUseCase = new BillingUseCase()


        sut = new Teste(
	testeRepository,
	billingUseCase,
	)
     })   
   }