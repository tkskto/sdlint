# npm distribution plan

## Status

npm distribution is planned but not implemented. sdlint is not currently published to the npm registry, so user-facing installation instructions must continue to use Cargo until a package has been published and verified.

Verified: 2026-09-21 (UTC).

## Distribution model

The Rust implementation remains the source of the sdlint executable. npm distributes precompiled executables; it does not replace Cargo as the build system.

Use one launcher package and separate platform packages:

```text
launcher package
├── JavaScript command wrapper exposed through package.json bin
└── optionalDependencies
    ├── macOS arm64 executable package
    ├── macOS x64 executable package
    ├── Linux x64 executable package
    ├── Linux arm64 executable package
    └── Windows x64 executable package
```

Each platform package contains one executable and restricts installation with the package.json os and cpu fields. Linux packages must also distinguish libc when both glibc and musl builds are supported. The launcher declares platform packages as optionalDependencies, locates the installed package for the current platform, and starts the executable while preserving arguments, standard streams, signals, and exit status.

The launcher and every platform package use the same version. Release automation publishes platform packages first and the launcher last, preventing the launcher from referring to versions that are not yet available.

Do not compile Rust code on the user's machine and do not download an executable from a separate host during installation. Shipping executables inside platform packages keeps installation reproducible, avoids requiring a Rust toolchain, and avoids an install-time network trust boundary beyond the npm registry.

The npm package.json bin field exposes package executables, while os, cpu, libc, and optionalDependencies provide the metadata needed for platform selection. See the [npm package.json specification](https://docs.npmjs.com/files/package.json/).

## Initial target matrix

| Operating system | Architecture | Runtime detail | Initial status |
| --- | --- | --- | --- |
| macOS | arm64 | native executable | required |
| macOS | x64 | native executable | required |
| Linux | x64 | glibc | required |
| Linux | arm64 | glibc | required |
| Windows | x64 | native executable | required |
| Linux | x64 and arm64 | musl | decision pending |
| Windows | arm64 | native executable | decision pending |

The launcher must report an actionable unsupported-platform error when no matching executable package is installed.

## User experience

After publication, the package should support global installation and one-off execution:

```console
npm install --global <package-name>
sdlint page.html
npx <package-name> page.html
```

The package name or scope has not been selected. Availability and ownership must be confirmed before it is recorded in installation documentation.

## Release and security requirements

Build each executable from the tagged source revision with Cargo.lock and the locked dependency graph. Verify that every executable reports the release version and can lint representative valid and invalid fixtures before packaging it.

Use npm trusted publishing from a GitHub-hosted runner so releases authenticate with short-lived OIDC credentials instead of a long-lived npm write token. Trusted publishing automatically produces provenance for a public package published from a public repository. It currently requires npm 11.5.1 or later, Node.js 22.14.0 or later, and id-token write permission in the publishing job. See [Trusted publishing for npm packages](https://docs.npmjs.com/trusted-publishers/) and [npm provenance](https://docs.npmjs.com/generating-provenance-statements/).

The release workflow must follow the repository security policy: pin every GitHub Action to a full commit SHA, grant contents read by default, grant id-token write only to the publishing job, and never expose publishing authority to untrusted pull request code. Prefer staged publishing if a manual approval boundary is desired.

Before publishing, inspect each package with npm pack and test installation from the generated archive. The published package must contain only the launcher or target executable and the metadata, license, and README required for that package.

## Versioning

Cargo.toml, the launcher package, and every platform package must have the same semantic version. A release must fail before publication if the versions differ.

Rule IDs and CLI behavior remain governed by the existing compatibility policy and CLI specification. Adding npm as a distribution channel must not create npm-specific command behavior.

## Decisions still required

- Choose an unscoped or scoped npm package name and confirm registry availability.
- Decide whether crates.io publication is part of the same release or remains independent.
- Decide whether Linux musl and Windows arm64 are required for the first release.
- Choose the minimum supported Node.js version for the launcher.
- Choose CommonJS or ECMAScript modules for the launcher.
- Define whether releases publish directly or use npm staged publishing with approval.

## Completion criteria

- All required platform packages install only on their declared targets.
- Global installation and npx both expose the sdlint command.
- Arguments, standard input, standard output, standard error, and exit codes match direct execution of the Rust binary.
- An unsupported platform produces a concise error that lists the supported targets.
- Platform packages and the launcher are published at one synchronized version.
- The release has npm provenance and does not require a long-lived npm publishing token.
- README installation instructions are updated only after the published package has been installed and smoke-tested from the registry.
