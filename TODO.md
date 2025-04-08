# TO DO

_**Private:** Do **not** merge into the `main` branch!_

-----

- [ ] integrate / watch-for the `spareirq` patch / branch
- [ ] see "2.1.3. Atomic Register Access" (pg 26)
  - [ ] I _think_ this means all writable APB and AHB peripherals
    - which means base addresses `0x40000000` and `0x50000000`, respectively 
      - see "2.2. Address Map" (pg 30)
    - not certain what the "self-hosted CoreSight window, including Arm Mem-AP" addresses are
    - the "3.7.5.1. Cortex-M33 EPPB Registers" (pg 224) **have** atomic register access
- [ ] what about `TrustZone` access, or is this handled by the `IDAU`?

