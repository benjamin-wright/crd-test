# ASYNC-TOOLS

Helpers for doing asynchronous things

## sleep

```js
// Sleep for one second
await sleep(1000);
```

## forEach

```js
// Async version of arr.forEach

await forEach(array, async element => await doSomething(element));
```