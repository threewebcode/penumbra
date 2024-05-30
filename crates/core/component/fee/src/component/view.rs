use anyhow::{anyhow, Result};
use async_trait::async_trait;
use cnidarium::{StateRead, StateWrite};
use penumbra_proto::{StateReadProto, StateWriteProto};

use crate::{params::FeeParameters, state_key, GasPrices};

/// This trait provides read access to fee-related parts of the Penumbra
/// state store.
#[async_trait]
pub trait StateReadExt: StateRead {
    /// Gets the fee parameters from the JMT.
    async fn get_fee_params(&self) -> Result<FeeParameters> {
        self.get(state_key::fee_params())
            .await?
            .ok_or_else(|| anyhow!("Missing FeeParameters"))
    }

    /// Gets the current gas prices for the fee token.
    async fn get_gas_prices(&self) -> Result<GasPrices> {
        // When we implement dynamic gas pricing, we will want
        // to read the prices we computed. But until then, we need to
        // read these from the _fee params_ instead, since those are
        // the values that will get updated by governance.
        let params = self.get_fee_params().await?;
        Ok(params.fixed_gas_prices)
    }

    /// Gets the current gas prices for alternative fee tokens.
    async fn get_alt_gas_prices(&self) -> Result<Vec<GasPrices>> {
        // When we implement dynamic gas pricing, we will want
        // to read the prices we computed. But until then, we need to
        // read these from the _fee params_ instead, since those are
        // the values that will get updated by governance.
        let params = self.get_fee_params().await?;
        Ok(params.fixed_alt_gas_prices)
    }

    /// Returns true if the gas prices have been changed in this block.
    fn gas_prices_changed(&self) -> bool {
        self.object_get::<()>(state_key::gas_prices_changed())
            .is_some()
    }
}

impl<T: StateRead + ?Sized> StateReadExt for T {}

#[async_trait]
pub trait StateWriteExt: StateWrite {
    /// Writes the provided fee parameters to the JMT.
    fn put_fee_params(&mut self, params: FeeParameters) {
        self.put(state_key::fee_params().into(), params);
        // This could have changed the gas prices, so mark them as changed.
        self.object_put(state_key::gas_prices_changed(), ());
    }

    /*
    We shouldn't be setting gas prices directly, until we have dynamic gas pricing.
    /// Writes the provided gas prices to the JMT.
    fn put_gas_prices(&mut self, gas_prices: GasPrices) {
        // Change the gas prices:
        self.put(state_key::gas_prices().into(), gas_prices);

        // Mark that they've changed
        self.object_put(state_key::gas_prices_changed(), ());
    }
     */
}

impl<T: StateWrite + ?Sized> StateWriteExt for T {}

#[async_trait]
pub(super) trait FeeWriteInner: StateWrite {
    fn accumulate_fee(&mut self, fee: Fee) {
    }
}

impl<T: StateWrite + ?Sized> FeeWriteInner for T {

}
