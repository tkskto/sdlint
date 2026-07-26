# WebSite rule sources

This file is the source record for rules concerning WebSite. Vocabulary semantics and Google Search feature requirements are kept in separate sections.

Verification date: 2026-07-26 (UTC). See the [source interpretation and maintenance policy](../rule-sources.md#source-interpretation).

## Definitions and requirements

### schema.org definition

| Type/property | Definition | Requirement | Target feature | Origin | Source / verified |
| --- | --- | --- | --- | --- | --- |
| WebSite | A set of related web pages, normally served from one domain. | none (vocabulary) | none; vocabulary semantics | schema.org-specific | [schema.org/WebSite](https://schema.org/WebSite), 2026-07-26 |
| name / alternateName / url | The site's primary name, alternative name, and URL. | none (vocabulary) | none | schema.org-specific | [name](https://schema.org/name), [alternateName](https://schema.org/alternateName), [url](https://schema.org/url), 2026-07-26 |

### Google Search Central requirements

Target feature: site name in Google Search (not a rich result).

| Property | Required/recommended | Google constraint | Origin | Source / verified |
| --- | --- | --- | --- | --- |
| name | required | Preferred site name. | Google-specific | [Site names](https://developers.google.com/search/docs/appearance/site-names), 2026-07-26 |
| url | required | Canonical home-page URL for the site. | Google-specific | same source, 2026-07-26 |
| alternateName | recommended | Backup site name; one or more alternatives may be supplied. | Google-specific | same source, 2026-07-26 |

Google expects the WebSite markup on the domain or subdomain home page; it is not a site-name mechanism for arbitrary subdirectories. Site names are generated automatically and markup is only one signal, so display is not guaranteed.
