use super::marlowe::*;

impl Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.currency_symbol.cmp(&other.currency_symbol) {
            std::cmp::Ordering::Equal => self.token_name.cmp(&other.token_name),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for Party {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Party::Address(a) = self {
            match other {
                Party::Address(b) => {
                    a.cmp(b)
                },
                _ => std::cmp::Ordering::Less
            }
        } else if let Party::Role {role_token } = self {
            let my_role_token = role_token;
            
            match other {
                Party::Role {role_token} => {
                    my_role_token.cmp(role_token)
                },
                _ => std::cmp::Ordering::Greater
            }
        } else {
            unreachable!()
        }

    }
}

impl PartialOrd for Party {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Address {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.is_mainnet.cmp(&other.is_mainnet) {
            std::cmp::Ordering::Equal => {
                let self_bech = self.as_bech32().ok();
                let other_bech = other.as_bech32().ok();
                match (self_bech, other_bech) {
                    (Some(sb), Some(ob)) => sb.cmp(&ob),
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            },
            ordering => ordering
        }
    }
}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl PartialOrd for ChoiceId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ChoiceId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.choice_owner.cmp(&other.choice_owner) {
            core::cmp::Ordering::Equal => { },
            order => return order
        }        

        self.choice_name.cmp(&other.choice_name)
        
    }
}
