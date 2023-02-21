# One shot outcall

This project demonstrates using one-shot messaging (i.e. [notify](https://docs.rs/ic-cdk/0.7.0/ic_cdk/api/call/fn.notify_with_payment128.html)) to make an [HTTP Outcall](https://internetcomputer.org/https-outcalls) without caring about the response.

This pattern could be useful, e.g. to deliver signed transactions to an RPC endpoint. You may want to use a timer to check for inclusion a couple of seconds later.

## How to run?

```
dfx start --clean --background`
dfx deploy
dfx canister call one_shot_outcall one_shot_outcall
```

You can see the result at: https://webhook.site/#!/fcee8cb5-ae12-40d2-95d2-05a899d74092/3068f018-e614-454b-b994-e3bfd75b3b0f/1

The URL to call is currently hardcoded to a webhook test site, but you can change the project easily e.g. to provide the URL as an argument to the call.



