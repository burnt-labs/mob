module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  roots: ['<rootDir>/src'],
  testMatch: ['**/*.test.ts'],
  collectCoverageFrom: [
    'src/**/*.ts',
    '!src/**/*.test.ts',
    '!src/generated/**'
  ],
  moduleFileExtensions: ['ts', 'js', 'json'],
  testTimeout: 30000,
};
