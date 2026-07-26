# BreadcrumbList rule sources

This file is the source record for rules concerning BreadcrumbList. Vocabulary semantics and Google Search feature requirements are kept in separate sections.

Verification date: 2026-07-26 (UTC). See the [source interpretation and maintenance policy](../rule-sources.md#source-interpretation).

## Definitions and requirements

### schema.org definition

| Type/property | Definition | Requirement | Target feature | Origin | Source / verified |
| --- | --- | --- | --- | --- | --- |
| BreadcrumbList | An ItemList made of linked web pages, normally described with their position in a hierarchy. | none (vocabulary) | none; vocabulary semantics | schema.org-specific | [schema.org/BreadcrumbList](https://schema.org/BreadcrumbList), 2026-07-26 |
| itemListElement | Items in the list; values may be ListItem, Text, or Thing. | none (vocabulary) | none | schema.org-specific | [schema.org/itemListElement](https://schema.org/itemListElement), 2026-07-26 |
| ListItem.item / name / position | The represented item, its name, and its position in an ordered list. position is an integer or text. | none (vocabulary) | none | schema.org-specific | [item](https://schema.org/item), [name](https://schema.org/name), [position](https://schema.org/position), 2026-07-26 |

### Google Search Central requirements

Target feature: Breadcrumb rich result.

| Property | Required/recommended | Google constraint | Origin | Source / verified |
| --- | --- | --- | --- | --- |
| itemListElement | required | Array of ListItem entries representing the breadcrumb trail. | Google-specific | [Breadcrumb structured data](https://developers.google.com/search/docs/appearance/structured-data/breadcrumb), 2026-07-26 |
| itemListElement.item | required except for the last item | Fully qualified URL of the page represented by the crumb; the last crumb may omit it. | Google-specific | same source, 2026-07-26 |
| itemListElement.name | required | User-visible breadcrumb title. | Google-specific | same source, 2026-07-26 |
| itemListElement.position | required | Position in the trail, starting with 1. | Google-specific | same source, 2026-07-26 |

Multiple trails may be supplied. This page is a dedicated rich-result document; display nevertheless remains at Google's discretion.
