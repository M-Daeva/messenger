# Messenger

Messenger is cosmwasm smart contract training project.

A user can post a message with any body and some tag (atom, osmo or juno). A message can be common, rare or epic. Any message has lifetime (common - 5 min, rare - 30 min, epic - forever) and cooldown (common - 1 min, rare - 2 min, epic - 1 min). Common message can be sent completely for free, rare requires some $MST tokens staked in protocol and epic message just requires to burn some $MST. A user can delete own messages, change message tag and edit body of rare and epic messages fo free. A contract instantiator can delete any message.

