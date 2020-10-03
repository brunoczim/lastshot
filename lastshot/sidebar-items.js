initSidebarItems({"fn":[["channel","Creates a new multiple producers multiple consumers (MPMC) \"last-shot\" channel."]],"struct":[["NoReceivers","Returned when there are no `Receiver`s connected."],["NoSenders","Returned when there are no `Sender`s connected."],["Receiver","A receiver handle of a MPMC last-shot channel. It receives messages from the channel. It can be freely cloned and if `T: Send`, also shared. The cost of cloning is the cost of incrementing two atomic variables."],["Sender","A sender handle of a MPMC last-shot channel. It sends messages messages from the channel. It can be freely cloned and if `T: Send`, also shared. The cost of cloning is the cost of incrementing two atomic variables. Sending a message always overwrite the previous message."]]});