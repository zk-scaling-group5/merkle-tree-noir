import fs from 'fs/promises';
import path from 'path';

// Circuits live in packages/noir. Only a circuit's Nargo.toml and its entry
// point are ever served, so an untrusted `filename` cannot walk out of the
// directory or read anything else off the server.
const CIRCUIT_ROOT = path.resolve(process.cwd(), '../noir');
const CIRCUIT_FILE = /^([A-Za-z0-9_-]+)\/(Nargo\.toml|src\/main\.nr)$/;

export default async function handler(req, res) {
  const { filename } = req.query;

  const requested = typeof filename === 'string' && CIRCUIT_FILE.exec(filename);
  if (!requested) {
    res.status(400).send({ error: 'Invalid filename' });
    return;
  }

  const [, circuit, file] = requested;
  const filePath = path.join(CIRCUIT_ROOT, circuit, file);

  try {
    const data = await fs.readFile(filePath, 'utf-8');
    res.status(200).send(data);
  } catch (error) {
    res.status(500).send({ error: 'Error reading file' });
  }
}
