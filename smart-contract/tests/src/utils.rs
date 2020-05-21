use casperlabs_contract::args_parser::ArgsParser;
use casperlabs_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casperlabs_types::{account::PublicKey, bytesrepr::FromBytes, CLTyped, Key, U512};

const WASM: &str = "contract.wasm";

pub mod account {
    use super::PublicKey;
    pub const ADMIN: PublicKey = PublicKey::ed25519_from([1u8; 32]);
    pub const ALI: PublicKey = PublicKey::ed25519_from([2u8; 32]);
    pub const BOB: PublicKey = PublicKey::ed25519_from([3u8; 32]);
}

pub struct Sender(pub PublicKey);

pub struct SmartContractContext {
    pub context: TestContext,
    pub contract_hash: Hash,
    pub indirect_hash: Hash,
    pub current_time: u64,
    pub contract_name: String,
}

impl SmartContractContext {
    pub fn deployed(
        indirect_name: &str,
        contract_name: &str,
        deploy_args: impl ArgsParser,
    ) -> Self {
        let clx_init_balance = U512::from(10_000_000_000u64);
        let mut context = TestContextBuilder::new()
            .with_account(account::ADMIN, clx_init_balance)
            .with_account(account::ALI, clx_init_balance)
            .with_account(account::BOB, clx_init_balance)
            .build();
        let code = Code::from(WASM);
        let session = SessionBuilder::new(code, deploy_args)
            .with_address(account::ADMIN)
            .with_authorization_keys(&[account::ADMIN])
            .with_block_time(0)
            .build();
        context.run(session);
        let contract_hash = Self::contract_hash(&context, contract_name);
        let indirect_hash = Self::contract_hash(&context, indirect_name);
        Self {
            context,
            contract_hash,
            indirect_hash,
            current_time: 0,
            contract_name: contract_name.to_string(),
        }
    }

    pub fn set_block_time(&mut self, block_time: u64) {
        self.current_time = block_time;
    }

    pub fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(account::ADMIN, &[&self.contract_name, &name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    pub fn call_indirect(&mut self, sender: Sender, args: impl ArgsParser) {
        let Sender(address) = sender;
        let code = Code::Hash(self.indirect_hash);
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .with_block_time(self.current_time)
            .build();
        self.context.run(session);
    }

    fn contract_hash(context: &TestContext, name: &str) -> Hash {
        let contract_ref: Key = context
            .query(account::ADMIN, &[name])
            .unwrap_or_else(|_| panic!("{} contract not found", name))
            .into_t()
            .unwrap_or_else(|_| panic!("{} is not a type Contract.", name));
        contract_ref
            .into_hash()
            .unwrap_or_else(|| panic!("{} is not a type Hash", name))
    }
}
