use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use crate::{BitSlot, Ipv4Prefix, Ipv6Prefix};

/// Shortening an Ip prefix
pub trait IpPrefixShortening {
    
    /// Shortens the prefix
    ///
    /// Remains unchanged if the specified length is greater
    /// than the previous one
    fn shorten(&mut self, maxlen: u8);
}

impl IpPrefixShortening for Ipv4Prefix 
{
    #[inline]
    fn shorten(&mut self, maxlen: u8) {
        if maxlen < self.len {
            self.addr &= u32::bitmask(maxlen);
            self.len = maxlen;
        }
    }
}

impl IpPrefixShortening for Ipv6Prefix
{
    #[inline]
    fn shorten(&mut self, maxlen: u8) {
        if maxlen < self.len {
            self.addr &= u128::bitmask(maxlen);
            self.len = maxlen;
        }
    }
}



impl IpPrefixShortening for IpNet {
    fn shorten(&mut self, maxlen: u8) {
        match self {
            IpNet::V4(ip) => ip.shorten(maxlen),
            IpNet::V6(ip) => ip.shorten(maxlen),
        }
    }
}

impl IpPrefixShortening for Ipv4Net 
{
    fn shorten(&mut self, maxlen: u8) 
    {
        if maxlen < self.prefix_len() {
            *self = Ipv4Net::new(self.network(), maxlen).unwrap()
        }
    }
}

impl IpPrefixShortening for Ipv6Net
{
    fn shorten(&mut self, maxlen: u8)
    {
        if maxlen < self.prefix_len() {
            *self = Ipv6Net::new(self.network(), maxlen).unwrap()
        }
    }
}