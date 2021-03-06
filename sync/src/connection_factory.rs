use std::sync::Arc;

use p2p::protocol::{LocalSyncNode, LocalSyncNodeRef, OutboundSyncConnectionRef,
                    InboundSyncConnectionRef};
use p2p::types::cn::CoreSyncData;
use p2p::types::PeerId;

use inbound_connection::InboundConnection;
use types::{LocalNodeRef, PeersRef};

pub struct ConnectionFactory {
    peers: PeersRef,
    local_node: LocalNodeRef,
}

impl ConnectionFactory {
    pub fn new(local_node: LocalNodeRef) -> ConnectionFactory {
        ConnectionFactory {
            peers: local_node.peers(),
            local_node,
        }
    }

    pub fn boxed(self) -> LocalSyncNodeRef {
        Box::new(self)
    }
}

impl LocalSyncNode for ConnectionFactory {
    fn new_sync_connection(&self,
                           peer_id: PeerId,
                           sync_data: &CoreSyncData,
                           connection: OutboundSyncConnectionRef)
                           -> InboundSyncConnectionRef {
        self.peers.insert(peer_id, sync_data, connection);
        self.local_node.on_connect(peer_id);

        Arc::new(InboundConnection::new(peer_id, self.peers.clone(), self.local_node.clone()))
    }
}
