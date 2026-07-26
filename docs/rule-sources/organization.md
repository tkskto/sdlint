# Organization rule sources

This file is the source record for rules concerning Organization. Vocabulary semantics and Google Search feature requirements are kept in separate sections.

Verification date: 2026-07-26 (UTC). See the [source interpretation and maintenance policy](../rule-sources.md#source-interpretation).

## Definitions and requirements

### schema.org definition

| Type/property | Definition | Requirement | Target feature | Origin | Source / verified |
| --- | --- | --- | --- | --- | --- |
| Organization | An organization such as a school, NGO, corporation, club, or business. | none (vocabulary) | none; vocabulary semantics | schema.org-specific | [schema.org/Organization](https://schema.org/Organization), 2026-07-26 |
| identity properties | name, alternateName, legalName, description, url, logo, and sameAs identify or describe the organization. | none (vocabulary) | none | schema.org-specific | [Organization properties](https://schema.org/Organization), 2026-07-26 |
| contact/location properties | address, contactPoint, email, telephone, and location describe contact or location data. | none (vocabulary) | none | schema.org-specific | same source, 2026-07-26 |
| identifiers/classification | identifier, leiCode, naics, taxID, vatID, globalLocationNumber, and iso6523Code carry identifiers or classifications. | none (vocabulary) | none | schema.org-specific | same source, 2026-07-26 |
| organizational facts | foundingDate, founder, employee, numberOfEmployees, parentOrganization, subOrganization, and related properties describe the organization and its relationships. | none (vocabulary) | none | schema.org-specific | same source, 2026-07-26 |

### Google Search Central requirements

Target feature: organization details used in Search, including logo and knowledge-panel understanding. This is a supported structured-data feature, but the document does not promise a separately named “Organization rich result.” That product fact must not be encoded as a required property.

| Property | Required/recommended | Google constraint | Origin | Source / verified |
| --- | --- | --- | --- | --- |
| name | required | Organization name. | Google-specific | [Organization structured data](https://developers.google.com/search/docs/appearance/structured-data/organization), 2026-07-26 |
| url | recommended | Organization home page URL. | Google-specific | same source, 2026-07-26 |
| logo | recommended | Representative logo URL or ImageObject; Google documents image constraints. | Google-specific | same source, 2026-07-26 |
| alternateName, description, email, telephone, sameAs | recommended | Additional identity/contact signals where applicable. | Google-specific | same source, 2026-07-26 |
| address, contactPoint | recommended | Structured postal/contact information where applicable. | Google-specific | same source, 2026-07-26 |
| legalName, foundingDate, numberOfEmployees | recommended | Additional organization facts where applicable. | Google-specific | same source, 2026-07-26 |
| globalLocationNumber, iso6523Code, leiCode, naics, taxID, vatID | recommended | Business identifiers where applicable. | Google-specific | same source, 2026-07-26 |
