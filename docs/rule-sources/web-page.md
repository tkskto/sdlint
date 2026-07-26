# WebPage rule sources

This file is the source record for rules concerning WebPage. Vocabulary semantics and Google Search feature requirements are kept in separate sections.

Verification date: 2026-07-26 (UTC). See the [source interpretation and maintenance policy](../rule-sources.md#source-interpretation).

## Definitions and requirements

### schema.org definition

| Type/property | Definition | Requirement | Target feature | Origin | Source / verified |
| --- | --- | --- | --- | --- | --- |
| WebPage | A web page; a page may be one element of a WebSite. | none (vocabulary) | none; vocabulary semantics | schema.org-specific | [schema.org/WebPage](https://schema.org/WebPage), 2026-07-26 |
| page identity | name, url, description, inLanguage, isPartOf, and mainEntity describe identity, language, containment, and principal subject. | none (vocabulary) | none | schema.org-specific | [WebPage properties](https://schema.org/WebPage), 2026-07-26 |
| page-specific properties | breadcrumb, datePublished, dateModified, primaryImageOfPage, speakable, and specialty describe page presentation or metadata. | none (vocabulary) | none | schema.org-specific | same source, 2026-07-26 |

### Google Search Central status

Google has no dedicated generic rich-result documentation for WebPage in the [Search Gallery](https://developers.google.com/search/docs/appearance/structured-data/search-gallery) (verified 2026-07-26; Google Search structured-data features; Google-specific). Consequently, sdlint defines no generic Google-required or Google-recommended properties for WebPage. Specialized subtypes/features can have their own Google requirements, and ordinary schema.org properties remain valid; the absence of a generic feature is a capability note, not a lint error.
