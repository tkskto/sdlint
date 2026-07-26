# Article rule sources

This file is the source record for rules concerning Article. Vocabulary semantics and Google Search feature requirements are kept in separate sections.

Verification date: 2026-07-26 (UTC). See the [source interpretation and maintenance policy](../rule-sources.md#source-interpretation).

## Definitions and requirements

### schema.org definition

| Type/property | Definition | Requirement | Target feature | Origin | Source / verified |
| --- | --- | --- | --- | --- | --- |
| Article | An article, such as a news article or investigative report; NewsArticle and BlogPosting are more specific types. | none (vocabulary) | none; vocabulary semantics | schema.org-specific | [schema.org/Article](https://schema.org/Article), 2026-07-26 |
| authorship/dates | author, datePublished, and dateModified state authorship and publication/modification dates. | none (vocabulary) | none | schema.org-specific | [Article properties](https://schema.org/Article), 2026-07-26 |
| presentation/content | headline, image, articleBody, articleSection, wordCount, and pagination properties describe the article and its presentation. | none (vocabulary) | none | schema.org-specific | same source, 2026-07-26 |

### Google Search Central requirements

Target feature: Article appearance in Google Search, including article title, image, and date information. Google states that there are no required properties for Article structured data; all listed enhancement properties are recommended. Thus their absence is never a Google-required-property error.

| Property | Required/recommended | Google constraint | Origin | Source / verified |
| --- | --- | --- | --- | --- |
| author (Person or Organization) | recommended | Author; provide author.name, and author.url when available. | Google-specific | [Article structured data](https://developers.google.com/search/docs/appearance/structured-data/article), 2026-07-26 |
| datePublished | recommended | First-publication date and time in ISO 8601 format. | Google-specific | same source, 2026-07-26 |
| dateModified | recommended | Most recent modification date and time in ISO 8601 format. | Google-specific | same source, 2026-07-26 |
| headline | recommended | Article title; concise titles are advised. | Google-specific | same source, 2026-07-26 |
| image | recommended | Representative image URL(s) or ImageObject; Google documents crawlability and image guidance. | Google-specific | same source, 2026-07-26 |

This is dedicated feature documentation, but Google does not guarantee an enhanced display and the page must still satisfy general policies.
