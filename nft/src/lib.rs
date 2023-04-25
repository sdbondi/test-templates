
use tari_template_lib::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Sparkle {
    pub brightness: u32,
}

#[template]
mod sparkle_nft_template {
    use super::*;

    pub struct SparkleNft {
        address: ResourceAddress,
    }

    impl SparkleNft {
        pub fn new() -> Self {
            // Create the non-fungible resource 
            let address = ResourceBuilder::non_fungible("SPKL").build();

            Self {
                address,
            }
        }

        pub fn mint(&mut self) -> Bucket {
            // Mint a new token with a random ID
            let id = NonFungibleId::random();
            self.mint_specific(id)
        }

        pub fn mint_specific(&mut self, id: NonFungibleId) -> Bucket {
            debug(format!("Minting {}", id));
            // These are characteristic of the NFT and are immutable
            let mut immutable_data = Metadata::new();
            immutable_data
                .insert("name", format!("Sparkle{}", id))
                .insert("image_url", format!("https://nft.storage/sparkle{}.png", id));

            // Mint the NFT, this will fail if the token ID already exists
            ResourceManager::get(self.address)
                .mint_non_fungible(id, &immutable_data, &Sparkle { brightness: 0 })
        }

        pub fn total_supply(&self) -> Amount {
            ResourceManager::get(self.address).total_supply()
        }

        pub fn inc_brightness(&mut self, id: NonFungibleId, brightness: u32) {
            debug(format!("Increase brightness on {} by {}", id, brightness));
            self.with_sparkle_mut(id, |data| {
                data.brightness = data.brightness.checked_add(brightness).expect("Brightness overflow");
            });
        }

        pub fn dec_brightness(&mut self, id: NonFungibleId, brightness: u32) {
            debug(format!("Decrease brightness on {} by {}", id, brightness));
            self.with_sparkle_mut(id, |data| {
                data.brightness = data
                    .brightness
                    .checked_sub(brightness)
                    .expect("Not enough brightness remaining");
            });
        }

        fn with_sparkle_mut<F: FnOnce(&mut Sparkle)>(&self, id: NonFungibleId, f: F) {
            let resource_manager = ResourceManager::get(self.address);
            let nft = resource_manager.get_non_fungible(&id);
            let mut data = nft.get_data::<Sparkle>();
            f(&mut data);
            resource_manager.update_non_fungible_data(id, &data);
        }

    }
}