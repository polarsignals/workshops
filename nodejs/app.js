const http = require("http");
const crypto = require("crypto");

// Intentionally slow: synchronous JSON parsing of a large payload
function buildLargeObject(n) {
  const obj = {};
  for (let i = 0; i < n; i++) {
    obj[`key_${i}`] = crypto.randomBytes(64).toString("hex");
  }
  return obj;
}

// This blocks the event loop — JSON.stringify + JSON.parse on a big object
function expensiveComputation() {
  const large = buildLargeObject(5000);
  const serialized = JSON.stringify(large);
  const parsed = JSON.parse(serialized);

  // Simulate more CPU work: sort all values
  const values = Object.values(parsed);
  values.sort();

  // Regex backtracking on a crafted string — classic CPU burner
  const evilRegex = /^(a+)+$/;
  evilRegex.test("a".repeat(24) + "!");

  return values.length;
}

const server = http.createServer((req, res) => {
  if (req.url === "/healthy") {
    res.writeHead(200);
    res.end("ok\n");
    return;
  }

  // Every request triggers the blocking computation
  const count = expensiveComputation();

  res.writeHead(200, { "Content-Type": "application/json" });
  res.end(JSON.stringify({ processed: count }));
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
  console.log(`Server listening on :${PORT}`);
  console.log(`Try: curl http://localhost:${PORT}/`);
  console.log(`Health: curl http://localhost:${PORT}/healthy`);
});
