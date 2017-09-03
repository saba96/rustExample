# rustExample

for installing rust please go this link:
https://doc.rust-lang.org/book/first-edition/getting-started.html

first compile the tcpclient.c file with this command:
gcc tcpclient.c

then open the get-mail folder and build and run with these commands:
cargo build
cargi clean

then open the mail-listener folder and build and run with these commands:
cargo build
cargi clean


tcpclient.c make a tcp connection to our mail-listener server 
you can send message via this client

mail-listener is a server waiting for tcp connections and save the mails to mongodb 

get-mail is a server for getting mails


you also need to have mongodb if you don't already have it.
go to this link for installation:
https://docs.mongodb.com/manual/installation/