use defmt::{info, panic};
use embassy_futures::join::join;
use embassy_rp::interrupt::typelevel::Binding;
use embassy_rp::usb::{Driver, Instance, InterruptHandler};
use embassy_rp::Peripheral;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::{Builder, Config};

pub async fn usb<'d, T: Instance>(
    usb_peripheral: impl Peripheral<P = T> + 'd,
    irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
) {
    let driver = Driver::new(usb_peripheral, irq);

    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Konkers");
    config.product = Some("Cuttlefish Console");
    config.serial_number = Some("12345678"); // TODO: replace with ID from flash chip.
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    // Buffers in which to store USB descriptors
    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];

    // USB control endpoint descriptor
    let mut control_buf = [0; 64];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );

    // Start building the USB device
    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);

    // Finish building USB device.
    let mut usb = builder.build();

    let usb_future = usb.run();
    let connection_future = async {
        loop {
            class.wait_connection().await;
            info!("USB Connected");
            let _ = console(&mut class).await;
            info!("USB Disconnected");
        }
    };
    join(usb_future, connection_future).await;
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn console<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {:x}", data);
        class.write_packet(data).await?;
    }
}
