import("rgb-solver").then(wasm => {

  let u = wasm.Universe.new(4,4);
  self.addEventListener("message", ev => {

    console.log("data", u.get_data());

    const num = Number(ev.data);
    if (!num) {
      self.postMessage({ allGood: false, error: ev.data + "is not a number!" });
      return;
    }
    try {
      const isPrime = wasm.is_prime(num);
      self.postMessage({ allGood: true, isPrime: isPrime });
    } catch (err) {
      self.postMessage({ allGood: false, error: err.message });
    }
  });
});
