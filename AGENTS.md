# Documentation Guidelines

Follow these rules when creating or editing Markdown documentation in this repository.

* In ordinary prose, do not insert line breaks within a paragraph; write each paragraph on one line. Use line breaks required by Markdown structure, such as headings, paragraph boundaries, lists, tables, and code blocks.
* Use backticks only for actual code fragments. Do not use them to decorate terms, type names, property names, file names, option names, or similar text.
* Do not use double-asterisk emphasis. Organize information with headings, tables, and document structure instead of decorative emphasis.
* Include only the specifications, rationale, and decision-making information that human readers need. Do not include authoring instructions, review procedures, or instructions for adding or splitting files in the document body.
* Separate source documentation by rule family and make each source accessible from an index. Do not conflate schema.org vocabulary definitions with Google Search Central requirements; record the source, verification date, target search feature, required or recommended status, and provider of feature-specific requirements for each.
* Explain separately when Google has no dedicated rich-result documentation or when a search feature has limited display eligibility; do not conflate those facts with property requirements or schema.org validity.

# Security Policy

Apply the following security policy in every session that works on this repository.

* Pin external GitHub Actions to full commit SHAs and verify the corresponding release and SHA when updating them.
* Do not add execution of untrusted code through pull_request_target, workflow_run, or Secrets.
* Grant GitHub Actions only the minimum permissions required for each job; do not give write permissions or tokens to jobs that do not need them.
* Update Cargo.lock when changing Cargo dependencies and use --locked in CI. Do not add unpinned Git dependencies or wildcard dependencies.
* Follow deny.toml for dependency vulnerabilities, licenses, and sources; do not add audit exclusions without a valid justification.
* Do not add secrets, access tokens, or personal information to source code, test data, logs, or CI output.
* Treat changes to CI configuration, dependency definitions, lockfiles, and audit configuration with greater care than ordinary code changes.
