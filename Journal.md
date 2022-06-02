# 25/5/22
### first entry on journal
- database interface is done, but not tested,
- im starting to plan on how to do the server part
- i will upload this shitty thing to github

my current idea is a server that i will call transceiver that
sends and recieve requests from the internet and the
[drumroll] _**SERVER ENGINE**_.
the server engine will recieve whatever the transceiver sends to it
and act upon it, with the help of the database interface, the transceiver 
should also have a module for auth, we'll figure auth later

- should probably use http for the transceiver, need to figure that out
- my idea for auth was using tokens, and sign requests with them
  - multiple clients can have the same token, but on deployment i will not 
  to that
  - using public key cryptography as usual, already implemented on a library 
  as it should be
  - the server will not accept requests that are not signed

**posdata:**
i was thinking on doing a TODO.md, but i will not do it for the time being, 
and just use this journal

# 02/6/22
### 1st entry
added small testing of the database interface on main function, i 
should prepare proper testing with a mock database

updated names on transaction table to price and payment

added update user balance delta