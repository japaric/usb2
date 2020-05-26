repr!(u8,
      /// Descriptor types
      Type {
    /// Device descriptor type
    Device = 1,
    /// Configuration descriptor type
    Configuration = 2,
    /// String descriptor type
    String = 3,
    /// Interface descriptor type
    Interface = 4,
    /// Endpoint descriptor type
    Endpoint = 5,
    /// Device qualifier descriptor type
    DeviceQualifier = 6,
    /// Other speed configuration descriptor type
    OtherSpeedConfiguration = 7,
    /// Interface power descriptor type
    InterfacePower = 8,
});
