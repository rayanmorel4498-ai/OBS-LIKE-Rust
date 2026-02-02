use crate::device_interfaces::i2c::I2CBus;
use super::battery::Battery;
use super::charging::Charger;
use super::fast_charging::FastCharging;
use super::wireless_charging::WirelessCharging;
pub struct PowerManagement<'a, B: I2CBus> {
    pub battery: Battery<'a, B>,
    pub charger: Charger<'a, B>,
    pub fast_charger: FastCharging<'a, B>,
    pub wireless_charger: WirelessCharging<'a, B>,
}
#[derive(Debug)]
pub enum PowerManagementError {
    Battery(super::battery::BatteryError),
    Charger(super::charging::ChargerError),
    FastCharge(super::fast_charging::FastChargeError),
    Wireless(super::wireless_charging::WirelessChargeError),
}
impl<'a, B: I2CBus> PowerManagement<'a, B> {
    pub fn new(
        battery: Battery<'a, B>,
        charger: Charger<'a, B>,
        fast_charger: FastCharging<'a, B>,
        wireless_charger: WirelessCharging<'a, B>,
    ) -> Self {
        PowerManagement {
            battery,
            charger,
            fast_charger,
            wireless_charger,
        }
    }
    pub fn init_all(&mut self) -> Result<(), PowerManagementError> {
        self.battery.init().map_err(PowerManagementError::Battery)?;
        self.charger.init().map_err(PowerManagementError::Charger)?;
        self.fast_charger.init().map_err(PowerManagementError::FastCharge)?;
        self.wireless_charger.init().map_err(PowerManagementError::Wireless)?;
        Ok(())
    }
    pub fn battery_voltage(&mut self) -> Result<u16, PowerManagementError> {
        self.battery.read_voltage().map_err(PowerManagementError::Battery)
    }
    pub fn battery_current(&mut self) -> Result<i16, PowerManagementError> {
        self.battery.read_current().map_err(PowerManagementError::Battery)
    }
    pub fn is_charging(&mut self) -> Result<bool, PowerManagementError> {
        self.charger.is_charging().map_err(PowerManagementError::Charger)
    }
    pub fn manage_charging(&mut self, battery_threshold: u8) -> Result<(), PowerManagementError> {
        let soc = self.battery.read_soc().map_err(PowerManagementError::Battery)?;
        if soc < battery_threshold {
            if !self.fast_charger.is_fast_charging().map_err(PowerManagementError::FastCharge)? {
                self.fast_charger.enable_fast_charge(2000, 9000)
                    .map_err(PowerManagementError::FastCharge)?;
            }
            self.charger.enable_charge().map_err(PowerManagementError::Charger)?;
            self.wireless_charger.disable().map_err(PowerManagementError::Wireless)?;
        } else {
            self.fast_charger.disable_fast_charge().map_err(PowerManagementError::FastCharge)?;
            self.charger.disable_charge().map_err(PowerManagementError::Charger)?;
            self.wireless_charger.enable().map_err(PowerManagementError::Wireless)?;
        }
        Ok(())
    }
    pub fn is_battery_critical(&mut self) -> Result<bool, PowerManagementError> {
        let soc = self.battery.read_soc().map_err(PowerManagementError::Battery)?;
        Ok(soc < 5)
    }
    pub fn emergency_shutdown(&mut self, temperature_c: u8) -> Result<(), PowerManagementError> {
        if temperature_c > 45 {
            self.charger.disable_charge().map_err(PowerManagementError::Charger)?;
            self.fast_charger.disable_fast_charge().map_err(PowerManagementError::FastCharge)?;
            self.wireless_charger.disable().map_err(PowerManagementError::Wireless)?;
        }
        Ok(())
    }
}
pub fn set_profile(profile: u32) -> Result<(), &'static str> {
    if profile > 2 {
        return Err("invalid_profile");
    }
    Ok(())
}