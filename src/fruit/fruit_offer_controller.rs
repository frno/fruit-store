use crate::fruit::FruitOffer;

/// Keep track of current offer and creating offer clones
pub(crate) struct FruitOfferController {
    offer: Option<FruitOffer>
}

impl FruitOfferController {
    pub(crate) fn new()-> FruitOfferController {
        return FruitOfferController { offer: None };
    }

    pub(crate) fn has_offer(&self) ->bool{
        return match self.offer {
            None => { false }
            Some(_) => { true }
        }
    }

    pub(crate) fn set_offer(&mut self, offer :&FruitOffer){
        self.offer = Some(offer.clone());
    }

    /// Returns None if Controller has no option else creates
    /// clone of the current offer
    pub(crate) fn get_offer(&self) ->Option<FruitOffer>{
        match &self.offer{
            None => {
                None
            }
            Some(offer) => {
                Some(FruitOffer{
                    fruit: offer.fruit.clone(),
                    price: offer.price,
                    start: offer.start.clone(),
                    duration_ms: offer.duration_ms,
                    id: offer.id
                })
            }
        }
    }
}
