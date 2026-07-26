# FAQPage rule sources

This file is the source record for rules concerning FAQPage. Vocabulary semantics and Google Search feature requirements are kept in separate sections.

Verification date: 2026-07-26 (UTC). See the [source interpretation and maintenance policy](../rule-sources.md#source-interpretation).

## Definitions and requirements

### schema.org definition

| Type/property | Definition | Requirement | Target feature | Origin | Source / verified |
| --- | --- | --- | --- | --- | --- |
| FAQPage | A WebPage presenting one or more frequently asked questions and answers. | none (vocabulary) | none; vocabulary semantics | schema.org-specific | [schema.org/FAQPage](https://schema.org/FAQPage), 2026-07-26 |
| mainEntity | The primary entity described by the page; for FAQ markup its values are Question nodes. | none (vocabulary) | none | schema.org-specific | [schema.org/mainEntity](https://schema.org/mainEntity), 2026-07-26 |
| Question.name / acceptedAnswer | The question text and the accepted Answer. | none (vocabulary) | none | schema.org-specific | [Question](https://schema.org/Question), 2026-07-26 |
| Answer.text | The answer text. | none (vocabulary) | none | schema.org-specific | [Answer](https://schema.org/Answer), 2026-07-26 |

### Google Search Central requirements and availability

Target feature: FAQ rich result. Google limits regular availability to well-known, authoritative government and health sites. This eligibility restriction is separate from property validation: a linter cannot prove site authority, and an otherwise correct FAQPage on another site is not thereby schema.org-invalid. The restriction SHOULD be reported only as informational eligibility guidance unless deployment context establishes it conclusively.

| Property | Required/recommended | Google constraint | Origin | Source / verified |
| --- | --- | --- | --- | --- |
| mainEntity | required | Array of Question objects comprising the FAQ. | Google-specific | [FAQ structured data](https://developers.google.com/search/docs/appearance/structured-data/faqpage), 2026-07-26 |
| mainEntity.name | required | Full text of each question. | Google-specific | same source, 2026-07-26 |
| mainEntity.acceptedAnswer | required | An Answer object for each question. | Google-specific | same source, 2026-07-26 |
| mainEntity.acceptedAnswer.text | required | Full answer; permitted HTML content must follow Google's documentation. | Google-specific | same source, 2026-07-26 |

The page must be authored as a FAQ with one answer per question; pages where users submit alternative answers belong to the separate Q&A feature. All FAQ content must be visible to users on the source page. There are no properties labelled recommended in Google's FAQ property table.
