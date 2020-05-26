// See section 9.3 of (USB2)
#[derive(Clone, Copy)]
pub struct bmRequestType {
    pub direction: Direction,
    pub ty: Type,
    pub recipient: Recipient,
}

impl bmRequestType {
    pub fn parse(bmrequesttype: u8) -> Result<Self, ()> {
        let direction = Direction::_from(bmrequesttype >> 7).ok_or(())?;
        let ty = Type::_from((bmrequesttype >> 5) & 0b11).ok_or(())?;
        let recipient = Recipient::_from(bmrequesttype & 0b1111).ok_or(())?;

        Ok(Self {
            direction,
            ty,
            recipient,
        })
    }
}

repr!(u8,
      /// Request direction
      Direction {
    /// Host to device
    HostToDevice = 0,
    /// Device to host
    DeviceToHost = 1,
});

repr!(u8,
      /// Request type
      Type {
    /// Standard request
    Standard = 0,
    /// Class-specific request
    Class = 1,
    /// Vendor-specific request
    Vendor = 2,
});

repr!(u8,
      /// Request recipient
      Recipient {
    /// Request recipient = device
    Device = 0,
    /// Request recipient = interface
    Interface = 1,
    /// Request recipient = endpoint
    Endpoint = 2,
    /// Request recipient = other
    Other = 3,
});
