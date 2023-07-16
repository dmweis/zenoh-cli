# (Unofficial) Zenoh CLI tools

Collection of CLI tools for working with [Zenoh](https://zenoh.io/) - [Zenoh repo](https://github.com/eclipse-zenoh/zenoh).  

For the most part these tools are based on the [examples](https://github.com/eclipse-zenoh/zenoh/tree/master/examples) in the [Zenoh github repository](https://github.com/eclipse-zenoh/zenoh).  

I mostly repackaged them so that I can build a simple debian package out of them and adopted some of them so they are easier to use.  
`z_pub` for example doesn't prepend the message ID to each message anymore.

## Running form source

Tools can be run using

```bash
cargo run --bin z_pub -- -k hello --value "it is i"
```

## Installing

Build debian package with `make build-deb`
If you want to build and install the package on the machine directly you can use `make install`

If you don't have `cargo-deb` installed you can install it with `make install-deps`

After install you can run tools by name:

```bash
z_pub  --key FOO --value BAR
```

## Warning

These tools are not guaranteed to be kept up to date! I recommend using the original example tools from the [Zenoh github repository](https://github.com/eclipse-zenoh/zenoh) under [examples](https://github.com/eclipse-zenoh/zenoh/tree/master/examples).  

## Disclaimer

_This software is not officially endorsed by the Eclipse Zenoh project or zettaScale!_

All product names, logos, and brands are property of their respective owners. All company, product and service names used in this website are for identification purposes only. Use of these names, logos, and brands does not imply endorsement.
