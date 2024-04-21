import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";


const TelepathyModule = buildModule("TelepathyModule", (m) => {
  const telepathy = m.contract("Telepathy");

  return { telepathy };
});

export default TelepathyModule;
