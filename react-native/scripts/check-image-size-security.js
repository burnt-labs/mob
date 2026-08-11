const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const os = require('node:os');
const path = require('node:path');
const { imageSize } = require('../third_party/image-size');
const { findBox } = require('../third_party/image-size/dist/types/utils.cjs');

const malformedIcns = Uint8Array.from([
  0x69, 0x63, 0x6e, 0x73,
  0x00, 0x00, 0x00, 0x10,
  0x69, 0x63, 0x30, 0x37,
  0x00, 0x00, 0x00, 0x00,
]);

assert.throws(
  () => imageSize(malformedIcns),
  /invalid ICNS image entry length/,
  'ICNS zero-length entries must be rejected',
);

const zeroLengthBox = Uint8Array.from([
  0x00, 0x00, 0x00, 0x00,
  0x6a, 0x78, 0x6c, 0x70,
]);

assert.throws(
  () => findBox(zeroLengthBox, 'jxlp', 0),
  /invalid image box size/,
  'JXL and HEIF zero-length boxes must be rejected',
);

async function checkFileApi() {
  const { imageSizeFromFile } = require('image-size/fromFile');
  assert.equal(
    typeof imageSizeFromFile,
    'function',
    'the Metro image-size/fromFile API must remain available',
  );

  const entryLength = 600 * 1024;
  const fileLength = 8 + entryLength;
  const largeIcns = Buffer.alloc(fileLength);
  largeIcns.write('icns', 0, 'ascii');
  largeIcns.writeUInt32BE(fileLength, 4);
  largeIcns.write('ic07', 8, 'ascii');
  largeIcns.writeUInt32BE(entryLength, 12);

  const tempDirectory = await fs.mkdtemp(path.join(os.tmpdir(), 'mob-image-size-'));
  const filePath = path.join(tempDirectory, 'large.icns');
  try {
    await fs.writeFile(filePath, largeIcns);
    assert.deepEqual(
      await imageSizeFromFile(filePath),
      { height: 128, type: 'icns', width: 128 },
      'the file API must accept valid ICNS entries beyond its read prefix',
    );
  } finally {
    await fs.rm(tempDirectory, { force: true, recursive: true });
  }
}

checkFileApi()
  .then(() => console.log('image-size denial-of-service regressions passed'))
  .catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
