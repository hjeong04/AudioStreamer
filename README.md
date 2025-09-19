# Milestones — Rust Audio Streamer (12 Weeks)

## One-time setup
- [ ] Install Rust toolchain: `rustup`, `rustc`, `cargo`
- [ ] Install helpful tools: `cargo-edit` (`cargo install cargo-edit`), `just` (optional)
- [ ] Git initialized with `.gitignore`, `README.md`, `LICENSE` (MIT/Apache-2.0)
- [ ] (Optional) Install Wireshark + tcpdump
- [ ] (Optional) Install Opus tools (`opus-tools`) and Docker (for broker/tests)
- [ ] (Optional) On Linux: grant capture permissions as needed (ALSA/pulse)

---

## Week 1 — Audio Capture, Framing & Timing
**Deliverable:** CLI captures mic to `.wav`; define minimal stream frame header.
**Networking concepts:** Bitrate math (sr × bit depth × channels), framing & endianness, basic latency budget.
**Success:** WAV plays cleanly; unit test passes for header encode/decode.

- [ ] `streamer record --out demo.wav`
- [ ] `sensor frame` module with header `{ver, ts, seq, len}`
- [ ] Unit tests for framing; log sample rate/format

---

## Week 2 — Playback & Clocking
**Deliverable:** CLI playback with adjustable buffer size.
**Networking concepts:** Producer/consumer timing, underflow/overflow, buffer sizing trade‑offs.
**Success:** Smooth playback without glitches; buffer flag impacts latency.

- [ ] `receiver play demo.wav --buffer-ms 80`
- [ ] Ring buffer abstraction for audio output
- [ ] Logging around underruns/overruns

---

## Week 3 — Local TCP Streaming (Loopback)
**Deliverable:** Mic → TCP → local receiver (127.0.0.1).
**Networking concepts:** TCP reliability/ordering, Nagle’s algorithm, delayed ACKs, backpressure.
**Success:** Live loopback audio; `--nodelay` toggling shows latency difference.

- [ ] `streamer serve --proto tcp --port 5000 [--nodelay]`
- [ ] `receiver connect --proto tcp --addr 127.0.0.1:5000`
- [ ] Length‑prefixed framing over TCP

---

## Week 4 — LAN TCP Streaming & Flow Control
**Deliverable:** Stream across two machines.
**Networking concepts:** MTU/MSS, head‑of‑line blocking, flow vs congestion control, Wireshark inspection.
**Success:** Stable LAN audio; Wireshark shows your app framing.

- [ ] Add reconnect/backoff
- [ ] Graceful shutdown (FIN), proper EOF handling
- [ ] Capture a short pcap for README

---

## Week 5 — UDP Mode & Packetization
**Deliverable:** UDP transport with fixed packetization (e.g., 20 ms PCM).
**Networking concepts:** Datagram semantics, fragmentation risk, latency vs loss.
**Success:** Lower latency vs TCP; occasional drops don’t crash playback.

- [ ] `--proto udp` + packet header `{seq, ts, payload}`
- [ ] Keep payload ≤ 1200B to avoid fragmentation
- [ ] Basic jitter buffer placeholder

---

## Week 6 — Jitter Buffer & Loss Handling
**Deliverable:** Adaptive jitter buffer; tolerate reordering/small losses.
**Networking concepts:** Jitter, reordering, playout delay control, `tc netem` simulation.
**Success:** Intelligible audio with injected 5–10% loss/jitter.

- [ ] Adaptive playout depth based on inter‑arrival variance
- [ ] Reordering window + late packet drop policy
- [ ] Metrics: loss %, reordering %, buffer depth

---

## Week 7 — RTP/RTCP (or RTP‑lite)
**Deliverable:** RTP packetization; optional RTCP receiver reports.
**Networking concepts:** RTP headers (SSRC, seq, timestamp), RTCP stats (loss, jitter), tooling support.
**Success:** Wireshark decodes stream as RTP; periodic RTCP stats printed.

- [ ] RTP header pack/unpack module
- [ ] Transport trait to support TCP/UDP under same API
- [ ] Minimal RTCP RR sender/receiver (optional)

---

## Week 8 — Opus Compression
**Deliverable:** Opus encode/decode at 32–64 kbps, 20 ms frames.
**Networking concepts:** Bitrate vs quality, FEC/PLC, DTX (silence suppression).
**Success:** ~20× bandwidth reduction vs PCM; good voice quality with loss.

- [ ] Opus encoder/decoder tasks
- [ ] RTP payload type mapping for Opus
- [ ] Configurable bitrate/frame size/channels

---

## Week 9 — Traversal Variant (WebSocket/TLS or QUIC)
**Deliverable:** WebSocket transport (or QUIC via `quinn`).
**Networking concepts:** NAT traversal, TLS, QUIC handshake & no HOL across streams.
**Success:** Receiver plays over WS/TLS from another network segment.

- [ ] WS server + client; simple auth token
- [ ] Self‑signed TLS for dev
- [ ] (Alt) QUIC path with `quinn`

---

## Week 10 — Multi‑Client Broadcast & Congestion Awareness
**Deliverable:** Fan‑out to multiple receivers; per‑client buffers.
**Networking concepts:** Unicast fan‑out vs UDP multicast; rate adaptation under slow clients.
**Success:** 2+ receivers within ~50–100 ms skew; bounded memory.

- [ ] Per‑client mpsc queues & drop policy
- [ ] Light telemetry: per‑client lag, drops
- [ ] Configurable max clients

---

## Week 11 — QoS/DSCP & Performance Tuning
**Deliverable:** Lower tail latency; optional DSCP marking.
**Networking concepts:** DSCP/WMM for voice, latency budget breakdown.
**Success:** Before/after latency histogram; DSCP visible in packet capture.

- [ ] `setsockopt` TOS/DSCP (where supported)
- [ ] Reduce allocations in hot paths; preallocated buffers
- [ ] Benchmarks for encode/decode & pipeline latency

---

## Week 12 — Observability, Packaging & Security
**Deliverable:** Polished CLI, metrics, docs, short demo video; optional DTLS/SRTP or TLS on WS.
**Networking concepts:** Operability (loss/jitter/bitrate/latency metrics), keying basics.
**Success:** Clean README with diagrams & pcaps; `v0.1.0` tagged release.

- [ ] `--help` polished with examples
- [ ] Feature flags: `rtp`, `opus`, `ws`, `quic`
- [ ] CI: fmt, clippy (deny warnings), test; release notes
