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

i should do a better job with the time, for now 
im storing seconds from epoch, a quick and dirty way of timestamps

i should write a facility for proofing table column names for having a 
robust type checked system and not just writing the names directly

# 23/8/22
### 1st entry
yes, i kinda abandoned this, creative block and lack of time, but i also had time to think more about this, 
and i came to a very important conclusion, i need to create my mock database before going any further, 
an in memory database without anything fancy, just the data properly organized, this requires moving all functions 
in the database module to traits so i can implement them on the sqlite (implementation next to the trait itself)
and the in memory database, also i need to revise all the code, that shouldnt be hard
i have to delay the sqlite db implementation until after i have everything else going.