use crate::socket::NetlinkSocket;
// use crate::packet::Kind;
// use crate::packet::MacAddr;
// use crate::packet::NetlinkPacket;
// use crate::packet::NetlinkErrorPacket;
// use crate::packet::NeighbourPacket;
// use crate::packet::RoutePacket;

use std::io;


pub struct Links<'a, 'b> {
    pub(crate) socket: &'a mut NetlinkSocket,
    pub(crate) buffer: &'b mut [u8],
}

impl<'a, 'b> Links<'a, 'b> {

}

impl<'a, 'b> Iterator for Links<'a, 'b> {
    type Item = Result<(), io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}