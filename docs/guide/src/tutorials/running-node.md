# Running a fullnode

In order to interact with the Penumbra network, users must provide an RPC URL
so that client software can read chain state and submit transactions.
This is true of the [Prax wallet], and of [pcli]. While users can select a publicly available
RPC URL from a party they trust, this guide demonstrates how a user can self-host
an RPC URL for use by themselves and others.

<!--
Plan:
  
  * just recommend hetzner
  * rent server
  * add dns
  * configure server (systemd configs from guide)
  * make sure to reference https
  * metrics are a stretch goal, could do that too via compose, ssh-port-forward to read safely
-->

## Renting a server

There are a variety of cloud providers that provide dedicated hardware for a per-month cost basis.
Generally, hardware-based solutions will have superior performance, particularly in storage latency,
and also more reliable performance over time. One suitable option is the
[Matrix AX52 by Hetzner](https://www.hetzner.com/dedicated-rootserver/ax52/).

To get started with Hetzner, [create an account](https://accounts.hetzner.com/signUp), provide billing information,
then request a dedicated hardware server. While preparing the server request,
you'll need to provide an SSH public key for the root user account. You can use this command to generate one
if you don't have one already:

```
ssh-keygen -t ed25519
cat ~/.ssh/id_ed25519.pub
```

Shortly after requesting the server, you should receive an email notifying you that it's ready to accept logins.

## Setting up DNS

In order to use HTTPS over the web interface, you'll need to create an A record for the domain you want to use,
pointing to the IPv4 address for the server. Visit the website for your DNS provider, and create the A record,
using the 

## Provisioning the server

Log into the server like so:

```
ssh -l root <YOUR_DNS_DOMAIN>
```

If that command fails, you'll need to debug your access settings. Then create a user account
for running the Penumbra software:

```
sudo useradd -m -d /home/penumbra penumbra -s /bin/bash
```

We'll use this account to configure the `pd` and `cometbft` data directories. First, 



[pcli]: ../pcli.md
[Prax wallet]: https://chromewebstore.google.com/detail/prax-wallet/lkpmkhpnhknhmibgnmmhdhgdilepfghe
