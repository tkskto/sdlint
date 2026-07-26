# Rule sources and compatibility policy

This inventory separates the schema.org vocabulary from Google Search Central feature eligibility. A schema.org-valid graph is not necessarily eligible for a Google search feature, and absence of a Google rich-result document does not make a schema.org type invalid.

Verification date: 2026-07-26 (UTC). “Required” and “recommended” below are the source's terms, not an inference by sdlint. Source URLs are recorded even when the build or test environment cannot access the network.

## Source interpretation

* schema.org-specific rows describe a vocabulary type, its range, or its domain. Schema.org generally does not label properties required or recommended, so the requirement column says “none (vocabulary)”.
* Google-specific rows describe eligibility or enhancement for the named Search feature. Google requirements are additional constraints, not changes to the schema.org vocabulary.
* Google documents say that required properties are necessary for eligibility, but valid markup does not guarantee display. Recommended properties improve the result or help Google understand it.

The common Google rules (JSON-LD, Microdata, or RDFa; correct representation of visible page content; no misleading or prohibited content; and compliance with Search policies) come from the [Structured data general guidelines](https://developers.google.com/search/docs/appearance/structured-data/sd-policies) (verified 2026-07-26; all structured-data search features; Google-specific; required for eligibility).

## Rule source index

| Rule family | Source record |
| --- | --- |
| BreadcrumbList | [BreadcrumbList](rule-sources/breadcrumb-list.md) |
| Organization | [Organization](rule-sources/organization.md) |
| WebSite | [WebSite](rule-sources/web-site.md) |
| WebPage | [WebPage](rule-sources/web-page.md) |
| Article | [Article](rule-sources/article.md) |
| FAQPage | [FAQPage](rule-sources/faq-page.md) |

## Rule ID and compatibility policy

Rule IDs are public API because they appear in output, suppressions, and CI configuration. They have this ASCII form:

```text
<origin>/<type>/<constraint>
```

* <origin> is schema for schema.org vocabulary constraints, google for Google Search feature constraints, or core for format-independent graph checks.
* <type> is the schema.org type in lower kebab case (breadcrumb-list, web-site, faq-page); cross-type checks use graph.
* <constraint> is a stable lower-kebab-case description, preferably ending in -required, -type, -format, or -value; it MUST NOT contain a severity.
* Examples: google/breadcrumb-list/position-required, schema/article/date-published-format, and core/graph/duplicate-node-id.

Once a Rule ID has shipped in a released version, it is never reused for a different semantic constraint, even after the original rule is removed. A renamed or materially redefined rule receives a new ID. Retired IDs remain reserved in the compatibility registry; they may be accepted temporarily as deprecated configuration aliases, but output always uses the new ID. Splitting one rule creates new IDs, and merging rules creates a new ID. Severity, wording, or source-date changes that do not change the tested predicate retain the ID. This prevents an old suppression from silently suppressing an unrelated future diagnostic.
