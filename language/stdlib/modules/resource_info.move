address 0x0:
module ResourceInfo {
  // return address which defines the module that contain struct `R`
  native public fun module_address<R: resource>(): address;
}