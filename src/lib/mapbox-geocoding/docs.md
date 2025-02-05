---
created: 2025-02-05T23:24:20 (UTC +11:00)
tags: []
source: https://docs.mapbox.com/api/search/geocoding/#geocoding-response-object
author:
---

# Geocoding API | API Docs | Mapbox

> ## Excerpt
>
> The Mapbox Geocoding API runs forward and reverse geocoding queries.

---

The **Mapbox Geocoding API** does two things: _forward geocoding_ and _reverse geocoding_:

- Forward geocoding converts location text into geographic coordinates, turning `2 Lincoln Memorial Circle SW` into `-77.050,38.889`.
- Reverse geocoding turns geographic coordinates into place names, turning `-77.050, 38.889` into `2 Lincoln Memorial Circle SW`. These location names can vary in specificity, from individual addresses to states and countries that contain the given coordinates.

For more background information on the Mapbox Geocoding API and how it works, see the [How geocoding works guide](https://docs.mapbox.com/help/getting-started/geocoding/#how-geocoding-works).

You may also use one of several [wrapper libraries](https://docs.mapbox.com/help/getting-started/geocoding/#libraries-and-plugins) to integrate the Mapbox Geocoding API into an application instead of using it directly.

- For [Mapbox Search JS SDK](https://docs.mapbox.com/mapbox-search-js/guides/), Geocoding components are available on releases `1.0.0-beta.20` and later.

[](https://docs.mapbox.com/playground/geocoding/)

## Geographic Feature Types[](https://docs.mapbox.com/api/search/geocoding/#geographic-feature-types "Direct link to Geographic Feature Types")

Various types of geographic features are available the Mapbox geocoding. Any type might appear as a top-level response, as context in a top-level response, or as a filtering option using the `types` parameter. Not all features are available or relevant in all parts of the world. New types are occasionally added as necessary to correctly capture global administrative hierarchies.

The geographic feature types listed from the largest to the most granular, are:

| Feature Type        | Description                                                                                                                                                                                                                                                                       |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `country`           | Generally recognized countries or, in some cases like Hong Kong, an area of quasi-national administrative status that has a designated country code under [ISO 3166-1](https://www.iso.org/iso-3166-country-codes.html).                                                          |
| `region`            | Top-level sub-national administrative features, such as states in the United States or provinces in Canada or China.                                                                                                                                                              |
| `postcode`          | Postal codes used in country-specific national addressing systems.                                                                                                                                                                                                                |
| `district`          | Features that are smaller than top-level administrative features but typically larger than cities, in countries that use such an additional layer in postal addressing (for example, prefectures in China).                                                                       |
| `place`             | Typically these are cities, villages, municipalities, etc. They’re usually features used in postal addressing, and are suitable for display in ambient end-user applications where current-location context is needed (for example, in weather displays).                         |
| `locality`          | Official sub-city features present in countries where such an additional administrative layer is used in postal addressing, or where such features are commonly referred to in local parlance. Examples include city districts in Brazil and Chile and arrondissements in France. |
| `neighborhood`      | Colloquial sub-city features often referred to in local parlance. Unlike `locality` features, these typically lack official status and may lack universally agreed-upon boundaries.                                                                                               |
| `street`            | Street features which host one or more addresses                                                                                                                                                                                                                                  |
| `block`             | Special feature type reserved for Japanese addresses.                                                                                                                                                                                                                             |
| `address`           | Individual residential or business addresses.                                                                                                                                                                                                                                     |
| `secondary_address` | Sub-unit, suite, or lot within a single parent address. Currently available in the US only.                                                                                                                                                                                       |

## Storing Geocoding Results[](https://docs.mapbox.com/api/search/geocoding/#storing-geocoding-results "Direct link to Storing Geocoding Results")

The Mapbox Geocoding API offers two types of result storage: **Permanent** and **Temporary**.

Temporary results are not allowed to be cached, while Permanent results are allowed to be cached and stored indefinitely.

Using Permanent storage with the Geocoding API requires that you have a [valid credit card on file](https://console.mapbox.com/account/settings/billing) or an active [enterprise contract](https://www.mapbox.com/contact/sales).

By default, the Geocoding API will use Temporary geocoding. To use Permanent geocoding, set the optional `permanent` parameter to `true`.

## Forward geocoding with search text input[](https://docs.mapbox.com/api/search/geocoding/#forward-geocoding-with-search-text-input "Direct link to Forward geocoding with search text input")

get

https://api.mapbox.com

/search/geocode/v6/forward?q={search_text}

The **forward geocoding** query type allows you to look up a location using a string of search text and returns its standardized address, geographic context, and coordinates.

| Required parameters | Type     | Description                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `q`                 | `string` | The feature you’re trying to look up. This could be an address, a city name, etc. The search text should be expressed as a URL-encoded UTF-8 string, and must not contain the semicolon character (either raw or URL-encoded). Your search text, once decoded, must consist of at most 20 words and numbers separated by spacing and punctuation, and at most 256 characters. |

The accuracy of coordinates returned by a forward geocoding request can be affected by how the addresses in the query are formatted. Learn more about address formatting best practices in the [Address geocoding format guide](https://docs.mapbox.com/help/troubleshooting/address-geocoding-format-guide/).

|
| `access_token` | `string` | All geocoding requests must include an [access token](https://docs.mapbox.com/help/glossary/access-token/). |

You can further refine the results of a forward geocoding query with the following optional parameters:

| Optional parameters | Type    | Description                                                                                                                                                                                                                                                                                                                                                         |
| ------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permanent`         | boolean | Specify whether you intend to store the results of the query (`true`) or not (`false`, default).                                                                                                                                                                                                                                                                    |
| `autocomplete`      | boolean | Specify whether to return autocomplete results (`true`, default) or not (`false`). When autocomplete is enabled, results will be included that start with the requested string, rather than responses that match it exactly. For example, a query for `India` might return both `India` and `Indiana` with autocomplete enabled, but only `India` if it’s disabled. |

When autocomplete is enabled, each user keystroke counts as one request to the Geocoding API. For example, a search for "Cali" would be reflected as four separate Geocoding API requests. To reduce the total requests sent, you can configure your application to only call the Geocoding API after a specific number of characters are typed.

|
| `bbox` | number | Limit results to only those contained within the supplied bounding box. Bounding boxes should be supplied as four numbers separated by commas, in `minLon,minLat,maxLon,maxLat` order. The bounding box cannot cross the 180th meridian.  
You can use the [Location Helper](https://labs.mapbox.com/location-helper/) to find a bounding box for use with this API. |
| `country` | string | Limit results to one or more countries. Permitted values are [ISO 3166 alpha 2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes separated by commas. |
| `format` | string | Specify the desired response format of results (`geojson`, default) or for backwards compatibility (`v5`). |
| `language` | string | Set the language of the text supplied in responses. Also affects result scoring, with results matching the user’s query in the requested language being preferred over results that match in another language. For example, an autocomplete query for things that start with `Frank` might return `Frankfurt` as the first result with an English (`en`) language parameter, but `Frankreich` (“France”) with a German (`de`) language parameter.

Options are

[IETF language tags](https://en.wikipedia.org/wiki/IETF_language_tag) comprised of a mandatory [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) and, optionally, one or more IETF subtags for country or script.

More than one value can also be specified, separated by commas. The first language in the list will be considered as the primary language and a response will be generated for it. For other languages, translations will be provided, see the

[translations section](https://docs.mapbox.com/api/search/geocoding/#translations).

For more information on which specific languages are supported, see the [language coverage section](https://docs.mapbox.com/api/search/geocoding/#language-coverage).

|
| `limit` | integer | Specify the maximum number of results to return. The default is `5` and the maximum supported is `10`. |
| `proximity` | string | Bias the response to favor results that are closer to this location. Provided as two comma-separated coordinates in `longitude,latitude` order, or the string `ip` to bias based on reverse IP lookup. |
| `types` | string | Filter results to include only a subset (one or more) of the available feature types. Options are `country`, `region`, `postcode`, `district`, `place`, `locality`, `neighborhood`, `street`, and `address`. Multiple options can be comma-separated.

For more information on the available types, see the

[geographic feature types section](https://docs.mapbox.com/api/search/geocoding/#geographic-feature-types). |
| `worldview` | string | Returns features that are defined differently by audiences that belong to various regional, cultural, or political groups. Available worldviews are: `ar`,`cn`,`in`,`jp`,`ma`,`rs`,`ru`,`tr`,`us`. If `worldview` is not set, the `us` worldview boundaries are returned by default.

For more information about using the `worldview` parameter, see the [worldviews section](https://docs.mapbox.com/api/search/geocoding/#worldviews).

|

### Example requests: Forward geocoding with search text input[](https://docs.mapbox.com/api/search/geocoding/#example-requests-forward-geocoding-with-search-text-input "Direct link to Example requests: Forward geocoding with search text input")

### Response: Forward geocoding with search text input[](https://docs.mapbox.com/api/search/geocoding/#response-forward-geocoding-with-search-text-input "Direct link to Response: Forward geocoding with search text input")

See the [Geocoding response object section](https://docs.mapbox.com/api/search/geocoding/#geocoding-response-object).

## Forward geocoding with structured input[](https://docs.mapbox.com/api/search/geocoding/#forward-geocoding-with-structured-input "Direct link to Forward geocoding with structured input")

get

https://api.mapbox.com

/search/geocode/v6/forward?address_number={address_number}&street={street}&...

**Structured Input** is a type of forward geocoding search that allows you to define the feature type of each element of the search query by type. This can increase the accuracy of results for well-formed datasets. To use Structured Input, the `q` parameter is dropped in favor of separate parameters for each feature type.

For best results, each element of the query must be assigned a feature type, and set `autocomplete` to `false`.

| Required parameters | Type     | Description                                                                                                 |
| ------------------- | -------- | ----------------------------------------------------------------------------------------------------------- |
| `access_token`      | `string` | All geocoding requests must include an [access token](https://docs.mapbox.com/help/glossary/access-token/). |

The following feature types can be defined in a Structured Input query:

| Optional parameters | Type     | Description                                                                                                                                                                                                                                                                       |
| ------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `address_line1`     | `string` | A string including address_number and street. These values can be provided as separate parameters `address_number` and `street` listed below.                                                                                                                                     |
| `address_number`    | `string` | The number associated with the house.                                                                                                                                                                                                                                             |
| `street`            | `string` | The name of the street in the address                                                                                                                                                                                                                                             |
| `block`             | `string` | In some countries like Japan, the block is a component in the address                                                                                                                                                                                                             |
| `place`             | `string` | Typically these are cities, villages, municipalities, etc. They’re usually features used in postal addressing, and are suitable for display in ambient end-user applications where current-location context is needed (for example, in weather displays).                         |
| `region`            | `string` | Top-level sub-national administrative features, such as states in the United States or provinces in Canada or China.                                                                                                                                                              |
| `postcode`          | `string` | Postal codes used in country-specific national addressing systems.                                                                                                                                                                                                                |
| `locality`          | `string` | Official sub-city features present in countries where such an additional administrative layer is used in postal addressing, or where such features are commonly referred to in local parlance. Examples include city districts in Brazil and Chile and arrondissements in France. |
| `neighborhood`      | `string` | Colloquial sub-city features often referred to in local parlance. Unlike locality features, these typically lack official status and may lack universally agreed-upon boundaries. Not available for reverse geocoding requests.                                                   |
| `country`           | `string` | Generally recognized countries or, in some cases like Hong Kong, an area of quasi-national administrative status that has a designated country code under [ISO 3166-1](https://www.iso.org/iso-3166-country-codes.html).                                                          |

You can further refine the results of a forward geocoding query with the following optional parameters:

| Optional parameters | Type    | Description                                                                                                                                                                                                                                                                                                                                                         |
| ------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permanent`         | boolean | Specify whether you intend to store the results of the query (`true`) or not (`false`, default).                                                                                                                                                                                                                                                                    |
| `autocomplete`      | boolean | Specify whether to return autocomplete results (`true`, default) or not (`false`). When autocomplete is enabled, results will be included that start with the requested string, rather than responses that match it exactly. For example, a query for `India` might return both `India` and `Indiana` with autocomplete enabled, but only `India` if it’s disabled. |

When autocomplete is enabled, each user keystroke counts as one request to the Geocoding API. For example, a search for "Cali" would be reflected as four separate Geocoding API requests. To reduce the total requests sent, you can configure your application to only call the Geocoding API after a specific number of characters are typed.

|
| `bbox` | number | Limit results to only those contained within the supplied bounding box. Bounding boxes should be supplied as four numbers separated by commas, in `minLon,minLat,maxLon,maxLat` order. The bounding box cannot cross the 180th meridian.  
You can use the [Location Helper](https://labs.mapbox.com/location-helper/) to find a bounding box for use with this API. |
| `country` | string | Limit results to one or more countries. Permitted values are [ISO 3166 alpha 2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes separated by commas. |
| `format` | string | Specify the desired response format of results (`geojson`, default) or for backwards compatibility (`v5`). |
| `language` | string | Set the language of the text supplied in responses. Also affects result scoring, with results matching the user’s query in the requested language being preferred over results that match in another language. For example, an autocomplete query for things that start with `Frank` might return `Frankfurt` as the first result with an English (`en`) language parameter, but `Frankreich` (“France”) with a German (`de`) language parameter.

Options are

[IETF language tags](https://en.wikipedia.org/wiki/IETF_language_tag) comprised of a mandatory [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) and, optionally, one or more IETF subtags for country or script.

More than one value can also be specified, separated by commas. The first language in the list will be considered as the primary language and a response will be generated for it. For other languages, translations will be provided, see the

[translations section](https://docs.mapbox.com/api/search/geocoding/#translations).

For more information on which specific languages are supported, see the [language coverage section](https://docs.mapbox.com/api/search/geocoding/#language-coverage).

|
| `limit` | integer | Specify the maximum number of results to return. The default is `5` and the maximum supported is `10`. |
| `proximity` | string | Bias the response to favor results that are closer to this location. Provided as two comma-separated coordinates in `longitude,latitude` order, or the string `ip` to bias based on reverse IP lookup. |
| `types` | string | Filter results to include only a subset (one or more) of the available feature types. Options are `country`, `region`, `postcode`, `district`, `place`, `locality`, `neighborhood`, `street`, and `address`. Multiple options can be comma-separated.

For more information on the available types, see the

[geographic feature types section](https://docs.mapbox.com/api/search/geocoding/#geographic-feature-types). |
| `worldview` | string | Returns features that are defined differently by audiences that belong to various regional, cultural, or political groups. Available worldviews are: `ar`,`cn`,`in`,`jp`,`ma`,`rs`,`ru`,`tr`,`us`. If `worldview` is not set, the `us` worldview boundaries are returned by default.

For more information about using the `worldview` parameter, see the [worldviews section](https://docs.mapbox.com/api/search/geocoding/#worldviews).

|

### Example request: Forward geocoding with structured input[](https://docs.mapbox.com/api/search/geocoding/#example-request-forward-geocoding-with-structured-input "Direct link to Example request: Forward geocoding with structured input")

### Response: Forward geocoding with structured input[](https://docs.mapbox.com/api/search/geocoding/#response-forward-geocoding-with-structured-input "Direct link to Response: Forward geocoding with structured input")

See the [Geocoding response object section](https://docs.mapbox.com/api/search/geocoding/#geocoding-response-object).

## Reverse geocoding[](https://docs.mapbox.com/api/search/geocoding/#reverse-geocoding "Direct link to Reverse geocoding")

get

https://api.mapbox.com

/search/geocode/v6/reverse?longitude={longitude}&latitude={latitude}

The **reverse geocoding** query type allows you to look up a pair of coordinates and returns the geographic features there, including a standardized address or place and full geographic context.

| Required parameters | Type     | Description                                                                                                 |
| ------------------- | -------- | ----------------------------------------------------------------------------------------------------------- |
| `longitude`         | `number` | The `longitude` decimal value from the geographic coordinate for the location being queried.                |
| `latitude`          | `number` | The `latitude` decimal value from the geographic coordinate for the location being queried.                 |
| `access_token`      | `string` | All geocoding requests must include an [access token](https://docs.mapbox.com/help/glossary/access-token/). |

You can further refine the results of a reverse geocoding query with the following optional parameters:

| Optional parameters | Type      | Description                                                                                                                                                          |
| ------------------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permanent`         | `boolean` | Specify whether you intend to store the results of the query (`true`) or not (`false`, default).                                                                     |
| `country`           | `string`  | Limit results to one or more countries. Permitted values are [ISO 3166 alpha 2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes separated by commas. |
| `language`          | `string`  | Specify the user’s language. This parameter controls the language of the text supplied in responses.                                                                 |

Options are [IETF language tags](https://en.wikipedia.org/wiki/IETF_language_tag) comprised of a mandatory [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) and, optionally, one or more IETF subtags for country or script.

Only one value can also be specified.

For more information on which specific languages are supported, see the [language coverage section](https://docs.mapbox.com/api/search/geocoding/#language-coverage).

|
| `limit` | `integer` | Specify the maximum number of results to return. The default is `1` and the maximum supported is `5`.

The default behavior in reverse geocoding is to return at most one feature at each of the multiple levels of the administrative hierarchy (for example, one address, one region, one country). Increasing the limit allows returning multiple features of the same type, but only for one type (for example, multiple `address` results). So, setting `limit` to a higher-than-default value requires specifying exactly one `types` parameter.

|
| `types` | `string` | Filter results to include only a subset (one or more) of the available feature types. Options are `country`, `region`, `postcode`, `district`, `place`, `locality`, `neighborhood`, `street`, and `address`. Multiple options can be comma-separated.

For more information on the available types, see the [geographic feature types section](https://docs.mapbox.com/api/search/geocoding/#geographic-feature-types).

|
| `worldview` | `string` | Returns features that are defined differently by audiences that belong to various regional, cultural, or political groups. Available worldviews are: `ar`,`cn`,`in`,`jp`,`ma`,`rs`,`ru`,`tr`,`us`. If `worldview` is not set, the `us` `worldview` boundaries are returned by default.

For more information about using the `worldview` parameter, see the [worldviews section](https://docs.mapbox.com/api/search/geocoding/#worldviews).

|

### Example request: Reverse geocoding[](https://docs.mapbox.com/api/search/geocoding/#example-request-reverse-geocoding "Direct link to Example request: Reverse geocoding")

### Response: Reverse geocoding[](https://docs.mapbox.com/api/search/geocoding/#response-reverse-geocoding "Direct link to Response: Reverse geocoding")

The API response for a reverse geocoding query returns a GeoJSON feature collection in Mapbox Geocoding Response format. For more details on how a response from the Geocoding API is formatted, see the [Geocoding response object section](https://docs.mapbox.com/api/search/geocoding/#geocoding-response-object).

## Batch geocoding[](https://docs.mapbox.com/api/search/geocoding/#batch-geocoding "Direct link to Batch geocoding")

post

https://api.mapbox.com

/search/geocode/v6/batch

The batch geocoding query type allows you to request up to 1000 forward or reverse geocoding queries in a single request.

Batch geocoding requests are formatted as a JSON object passed as the `BODY` of the request, but with multiple search queries one after another. Fields which were defined as query parameters in the URL string will become fields in the JSON object. Fields specifying multiple values (for example `types`, `country`, `bbox`, `proximity`) can be passed either as a comma-separated strings or as JSON-formatted arrays (for example a types filter value could be represented as `"address,street,place"` or `["address", "street", "place"]`).

In a single batch geocoding request, you can bundle queries of different types, including forward queries, forward Structure Input queries, and reverse queries. Each query can also has parameters defined individually -- for example one query could have IP proximity set to on while the rest have it off.

You can make a _batch geocoding request_ by using an `HTTP POST` request with the following body structure:

Request Body

| Required parameters | Type     | Description                                                                                                 |
| ------------------- | -------- | ----------------------------------------------------------------------------------------------------------- |
| `access_token`      | `string` | All geocoding requests must include an [access token](https://docs.mapbox.com/help/glossary/access-token/). |

| Optional parameters | Type      | Description                                                                                 |
| ------------------- | --------- | ------------------------------------------------------------------------------------------- |
| `permanent`         | `boolean` | Specify if you intend to store the results of the query (`true`) or not (`false`, default). |

### Example request: Batch geocoding[](https://docs.mapbox.com/api/search/geocoding/#example-request-batch-geocoding "Direct link to Example request: Batch geocoding")

### Response: Batch geocoding[](https://docs.mapbox.com/api/search/geocoding/#response-batch-geocoding "Direct link to Response: Batch geocoding")

The response from a batch geocoding query is an object with a `batch` property. The batch property is an array of [Geocoding response objects](https://docs.mapbox.com/api/search/geocoding/#geocoding-response-object). The order of queries in the request determines the order of corresponding objects in the batch array. If there are no results returned for a particular query within the batch, the `features` array for that query is empty ( `"features": []` ).

Here's an example request in which there is a list of three queries. The first two requests are for forward geocoding, and the last one is for reverse.

Notice that in the response, the results are in same the order as the queries in the request, where the second query `aslkdjf` doesn't return a result but still exists in the second position of the array:

## Geocoding response object[](https://docs.mapbox.com/api/search/geocoding/#geocoding-response-object "Direct link to Geocoding response object")

The response to a Geocoding API request is an object that contains the following properties:

| Property   | Type     | Description                                                                                                  |
| ---------- | -------- | ------------------------------------------------------------------------------------------------------------ |
| `type`     | `string` | `"FeatureCollection"`, a GeoJSON type from the [GeoJSON specification](https://tools.ietf.org/html/rfc7946). |
| `features` | `array`  | An array of feature objects.                                                                                 |

**Forward geocodes:** Returned features are ordered by relevance.  
**Reverse geocodes:** Returned features are ordered by index hierarchy, from most specific features to least specific features that overlap the queried coordinates.
Read the [Search result prioritization](https://docs.mapbox.com/help/getting-started/geocoding/#search-result-prioritization) guide to learn more about how returned features are organized in the Geocoding API response.

|
| `attribution` | `string` | Attributes the results of the Mapbox Geocoding API to Mapbox. |

The data powering the Mapbox Geocoding API is constantly being updated and improved. This means that the values of properties in the response object are not guaranteed and may change within the same version of the API. Properties may be added to, but will not be removed from, the response within the same API version.

Each feature object in the `features` array may have the properties described below:

| Property               | Type     | Description                                                                                                                                               |
| ---------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`                   | `string` | Feature id. This property is named "id" to conform to the GeoJSON specification, but is the same id referred to as `mapbox_id` elsewhere in the response. |
| `type`                 | `string` | `"Feature"`, a GeoJSON type from the [GeoJSON specification](https://tools.ietf.org/html/rfc7946).                                                        |
| `geometry`             | `object` | An object describing the spatial geometry of the returned feature.                                                                                        |
| `geometry.type`        | `string` | `"Point"`, a GeoJSON type from the [GeoJSON specification](https://tools.ietf.org/html/rfc7946).                                                          |
| `geometry.coordinates` | `array`  | An array in the format `[longitude,latitude]` at the center of the specified `bbox`.                                                                      |
| `properties`           | `object` | An object containing the resulting feature's details.                                                                                                     |

Each properties object in a `feature` may have the attributes described below:

| Property                                                                                                                  | Type     | Description                                                                                                                                                                                                                                                                                                                  |
| ------------------------------------------------------------------------------------------------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mapbox_id`                                                                                                               | `string` | Feature id. The `mapbox_id` uniquely identifies a place in the Mapbox search database. Mapbox ID’s are accepted in requests to the Geocoding API as a forward search, and will return the feature corresponding to that id.                                                                                                  |
| `feature_type`                                                                                                            | `string` | A string describing the type of the feature. Options are `country`, `region`, `postcode`, `district`, `place`, `locality`, `neighborhood`, `street`, `address`.                                                                                                                                                              |
| _Formerly `place_type` in v5._                                                                                            |
| `name`                                                                                                                    | `string` | Formatted string of address_number and street.                                                                                                                                                                                                                                                                               |
| `name_preferred`                                                                                                          | `string` | The canonical or otherwise more common alias for the feature name. For example, searching for "America" will return "America" as the name, and "United States" as `name_preferred`.                                                                                                                                          |
| `place_formatted`                                                                                                         | `string` | Formatted string of result context: place region country postcode. The part of the result which comes after `name`.                                                                                                                                                                                                          |
| `full_address`                                                                                                            | `string` | Full formatted string of the feature, combining name_preferred and place_formatted.                                                                                                                                                                                                                                          |
| `context`                                                                                                                 | `object` | A [context object](https://docs.mapbox.com/api/search/geocoding/#the-context-object) is an object representing the hierarchy of encompassing parent features. This may include a sub-object for any of the following properties: `country`, `region`, `postcode`, `district`, `place`, `locality`, `neighborhood`, `street`. |
| _Which sub-objects are included is dependent upon the data coverage available and applicable to a given country or area._ |
| `coordinates`                                                                                                             | `object` | An object representing the geographical position and accuracy of the feature any routable points.                                                                                                                                                                                                                            |
| `coordinates.longitude`                                                                                                   | `number` | Longitude of result                                                                                                                                                                                                                                                                                                          |
| `coordinates.latitude`                                                                                                    | `number` | Latitude of result                                                                                                                                                                                                                                                                                                           |
| `coordinates.accuracy`                                                                                                    | `string` | Accuracy metric for a returned `address`\-`type` result. See [Point accuracy for address features](https://docs.mapbox.com/api/search/geocoding/#point-accuracy-for-address-features) below.                                                                                                                                 |
| `coordinates.routable_points`                                                                                             | `array`  | An array of routable point objects for an address feature, each including `name`, `longitude`, and `latitude` properties.                                                                                                                                                                                                    |
| `bbox`                                                                                                                    | `array`  | The bounding box of the feature as an array of `[minLon,minLat,maxLon,maxLat]`. This property is only provided with features of type `country`, `region`, `postcode`, `district`, `place`, `locality`, or `neighborhood`.                                                                                                    |
| `match_code`                                                                                                              | `object` | Additional metadata indicating how the result components match to the input query. See [Smart Address Match](https://docs.mapbox.com/api/search/geocoding/#smart-address-match) below.                                                                                                                                       |

### The Context Object[](https://docs.mapbox.com/api/search/geocoding/#the-context-object "Direct link to The Context Object")

The `context` object within `properties` is a powerful piece of data which breaks out the complete geographical hierarchy for a given address or place. It is both a reliable way to access the named values of each component part of an address, plus contains feature-specific data such as the Wikidata id and 3-letter alpha country code.

Each sub-object in the context always has a `mapbox_id` and `name` associated with it. The id can be queried directly via a forward geocoding search to traverse into a different geographical layer. Address features will also include an `address` sub-object, which additionally contains `street_name` and `address_number` properties.

A `secondary_address` feature will also have a matching sub-object in the context:

#### Example Context Object[](https://docs.mapbox.com/api/search/geocoding/#example-context-object "Direct link to Example Context Object")

The following is an example of the context object returned for the address `2595 Lucky John Drive, Park City, Utah 84060, United States`:

#### Translations[](https://docs.mapbox.com/api/search/geocoding/#translations "Direct link to Translations")

When providing more than one language in the context object for the `country`, `region`, `district`, `place`, `neighborhood` properties will appear the translations property. This is an object where the key is the language code and the value is an object with language and name properties.

##### Translations Example[](https://docs.mapbox.com/api/search/geocoding/#translations-example "Direct link to Translations Example")

Example of the context object when passing the parameter `language=en,es`.

### Point accuracy for `address` features[](https://docs.mapbox.com/api/search/geocoding/#point-accuracy-for-address-features "Direct link to point-accuracy-for-address-features")

The `coordinates.accuracy` property in a Geocoding response object is a point accuracy metric for the returned `address` feature. This list is subject to change.

| Accuracy       | Description                                                                         |
| -------------- | ----------------------------------------------------------------------------------- |
| `rooftop`      | Result intersects a known building/entrance.                                        |
| `parcel`       | Result is associated with one or more address within a specified polygonal boundary |
| `point`        | Result is a known address point but does not intersect a known rooftop/parcel.      |
| `interpolated` | Result position and existence are estimated based on nearby known addresses.        |
| `approximate`  | Result position is approximated by a 9-digit zipcode centroid.                      |
| `intersection` | For street type features only. The result is an intersection of 2 streets.          |

### Smart Address Match[](https://docs.mapbox.com/api/search/geocoding/#smart-address-match "Direct link to Smart Address Match")

The `match_code` object in the Geocoding API helps you understand how the resulting address feature aligns with the query submitted. Available only for address-type features, the `match_code` provides a breakdown of how each element of the result matches with the query, plus a confidence score, based on how well it matches. This can help you make decisions about what results to keep or throw out based on your application's tolerance for fuzzy matching on the query.

Smart Address Match is available for all forward geocoding requests that return an `address` type feature. It works best when using [Structured Input](https://docs.mapbox.com/api/search/geocoding/#forward-geocoding-with-structured-input) forward queries, as the request components must be typed explicitly.

| Confidence Score | Description                                                                                                                                                                                            |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `exact`          | No components are unmatched (up to 2 may be inferred) and no there are no extraneous query tokens.                                                                                                     |
| `high`           | One component (excluding `house_number` or `region`) may have been corrected. Additionally, if only `house_number`, `street`, and `postcode` are provided and match, high confidence is returned.      |
| `medium`         | Two components (excluding `house_number` or `region`) may have changed. Allows for minor misspellings. If `house_number`, `street`, `place`, and `postcode` are matched the `region` may be corrected. |
| `low`            | `house_number`, `region`, or more than 2 other components have been corrected.                                                                                                                         |

| Match Code       | Description                                                                                                                                                                                                                                                                                                                                                                                                  |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `matched`        | The component value matches the user's input.                                                                                                                                                                                                                                                                                                                                                                |
| `unmatched`      | The component value doesn't match the user's input, or the user didn't submit this component type as part of the query.                                                                                                                                                                                                                                                                                      |
| `not_applicable` | The component is not used in the postal address string for example `locality`.                                                                                                                                                                                                                                                                                                                               |
| `inferred`       | The component type wasn't submitted as part of the query, but we were able to confidently fill in the value. only returned for the `country` component.                                                                                                                                                                                                                                                      |
| `plausible`      | Only relevant for the `address_number` and `secondary_address` components. In the case of `address_number`, this means the address accuracy is interpolated. In the case of `secondary_address`, this means the secondary address was extrapolated, i.e. the primary address is known to have secondary addresses, but the geocoding service did not find a specific matching secondary address in our data. |

#### Example match_code[](https://docs.mapbox.com/api/search/geocoding/#example-match_code "Direct link to Example match_code")

This forward geocoding request with structured input includes a valid street address in Park City Utah, USA.

`https://api.mapbox.com/search/geocode/v6/forward?&address_number=2595&street=lucky john dr&place=park city&region=CO`

Note that the `region` parameter value is `CO`, and will yield an `unmatched` status in the `match_code` in the resulting feature's properties as the matched address is not in the US state of Colorado.

### Example response: Forward geocoding[](https://docs.mapbox.com/api/search/geocoding/#example-response-forward-geocoding "Direct link to Example response: Forward geocoding")

## Worldviews[](https://docs.mapbox.com/api/search/geocoding/#worldviews "Direct link to Worldviews")

Use the optional `worldview` parameter to identify geographic features whose characteristics are defined differently by audiences belonging to various regional, cultural, or political groups. The Geocoding API only supports worldviews for the `country` and `region` data types, but may expand to additional data types in the future.

### Available worldviews[](https://docs.mapbox.com/api/search/geocoding/#available-worldviews "Direct link to Available worldviews")

The Geocoding API supports the following values for the `worldview` parameter:

| Value | Description                              |
| ----- | ---------------------------------------- |
| `ar`  | Features for an Argentinian audience     |
| `cn`  | Features for a mainland Chinese audience |
| `in`  | Features for an Indian audience          |
| `jp`  | Features for a Japanese audience         |
| `ma`  | Features for a Moroccan audience         |
| `rs`  | Features for a Serbian audience          |
| `ru`  | Features for a Russian audience          |
| `tr`  | Features for a Turkish audience          |
| `us`  | Features for an American audience        |

_Returned features are not officially approved for use by the above countries' respective governments._

Each query supports a maximum of one worldview. Attempting to query for multiple worldviews in one search will result in an error message. When no worldview is selected, the Geocoding API returns features for the `us` worldview.

### Using the `worldview` parameter[](https://docs.mapbox.com/api/search/geocoding/#using-the-worldview-parameter "Direct link to using-the-worldview-parameter")

In Mapbox services, the [worldview](https://docs.mapbox.com/help/glossary/worldview/) data field is used to identify geographic features whose characteristics are defined differently by audiences belonging to various regional, cultural, or political groups. Setting a worldview affects the place values returned for an address that is located in an area of non-unanimous ownership or definition.

For example, consider an address in the Ilemi Triangle, which borders Eastern Equatoria (South Sudan) and Turkana (Kenya). Depending on the worldview selected, an address in this location will be returned in different country contexts:

- If `worldview=cn`, Kaaleng, Eastern Equatoria, or South Sudan will be returned.
- If `worldview=us`, Kaaleng, Turkana, or Kenya will be returned.

This means that if you use `wordlview=cn` worldview with the Kenya (ke) country filter, a search for an address in this area will not be successful. Instead, you would need to use `worldview=cn` in combination with the South Sudan (ss) country filter.

Be cautious when combining `worldview` with the `country` parameter, since these indices are only loosely coupled.

For a visual example of how changing the `worldview` value impacts the boundaries on a map, see the [companion Maps boundary service](https://docs.mapbox.com/mapbox-gl-js/example/toggle-worldviews/).

## Language coverage[](https://docs.mapbox.com/api/search/geocoding/#language-coverage "Direct link to Language coverage")

Translation availability also varies by language and region. We provide the following tiers of language support:

### Global coverage[](https://docs.mapbox.com/api/search/geocoding/#global-coverage "Direct link to Global coverage")

These languages are almost always present for `country`, `region`, and prominent `place` features.

| Language    |              |              |
| ----------- | ------------ | ------------ |
| `de` German | `en` English | `es` Spanish |
| `fr` French | `it` Italian | `nl` Dutch   |
| `pl` Polish | —            | —            |

### Local coverage[](https://docs.mapbox.com/api/search/geocoding/#local-coverage "Direct link to Local coverage")

These languages may lack global coverage but are almost always present for `country`, `region`, and prominent `place` features where they are widely used.

| Language                     |                               |                            |
| ---------------------------- | ----------------------------- | -------------------------- |
| `az` Azerbaijani             | `bn` Bengali                  | `ca` Catalan               |
| `cs` Czech                   | `da` Danish                   | `el` Modern Greek          |
| `et` Estonian                | `fa` Persian                  | `fi` Finnish               |
| `ga` Irish                   | `hu` Hungarian                | `id` Indonesian            |
| `is` Icelandic               | `ja` Japanese                 | `ka` Georgian              |
| `km` Central Khmer           | `ko` Korean                   | `lt` Lithuanian            |
| `lv` Latvian                 | `mk` Macedonian               | `mn` Mongolian             |
| `ms` Malay macrolanguage     | `nb` Norwegian Bokmål         | `pt` Portuguese            |
| `ro` Romanian                | `sk` Slovak                   | `sl` Slovenian             |
| `sq` Albanian                | `th` Thai                     | `tl` Tagalog               |
| `uk` Ukrainian               | `vi` Vietnamese               | `zh` Chinese               |
| `zh_Hans` Simplified Chinese | `zh_Hant` Traditional Chinese | `zh_TW` Taiwanese Mandarin |

### Limited coverage[](https://docs.mapbox.com/api/search/geocoding/#limited-coverage "Direct link to Limited coverage")

These languages are sometimes present but coverage tends to be inconsistent or geographically constrained.

| Language     |              |              |
| ------------ | ------------ | ------------ |
| `ar` Arabic  | `bs` Bosnian | `he` Hebrew  |
| `hi` Hindi   | `kk` Kazakh  | `lo` Lao     |
| `my` Burmese | `ru` Russian | `sr` Serbian |
| `sv` Swedish | `te` Telugu  | `tk` Turkmen |
| `tr` Turkish | —            | —            |

Find more details about the format and properties of Geocoding API responses in the [Geocoding response object section](https://docs.mapbox.com/api/search/geocoding/#geocoding-response-object).

## Geocoding API errors[](https://docs.mapbox.com/api/search/geocoding/#geocoding-api-errors "Direct link to Geocoding API errors")

| Response body `message`          | HTTP error code | Description                                                                                                       |
| -------------------------------- | --------------- | ----------------------------------------------------------------------------------------------------------------- |
| `Not Authorized - No Token`      | `401`           | No token was used in the query.                                                                                   |
| `Not Authorized - Invalid Token` | `401`           | Check the access token you used in the query.                                                                     |
| `Forbidden`                      | `403`           | There may be an issue with your account. Check your [Account page](https://console.mapbox.com/) for more details. |

In some cases, using an access tokens with URL restrictions can also result in a `403` error. For more information, see the [URL Restrictions](https://docs.mapbox.com/accounts/guides/tokens/#url-restrictions) in the Token management guide.

|
| `Not Found` | `404` | Check the endpoint you used in the query. |
| `Not Found` | `404` | No search text or structured input parameters were provided in the query. |
| `BBox is not valid. Must be an array of format [minX, minY, maxX, maxY]` | `422` | Check the format for `bbox`, where the first coordinate pair should refer to the southwestern corner of the box and the second pair should refer to the northeastern corner of the box. |
| `BBox {minX/maxX} value must be a number between -180 and 180` | `422` | Check the format of `minX` and `maxX` for `bbox`. |
| `BBox {minY/maxY} value must be a number between -90 and 90` | `422` | Check the format of `minY` and `maxY` for `bbox`. |
| `BBox {minX/minY} value cannot be greater than {maxX/maxY} value` | `422` | Check the values of the coordinate pairs used in `bbox`. |
| `Type "{input}" is not a known type. Must be one of: country, region, place, district, postcode, locality, neighborhood, and address` | `422` | Check the [`type`](https://docs.mapbox.com/api/search/geocoding/#geographic-feature-types) you used in the query. |
| `Stack "{input}" is not a known stack. Must be one of: …` | `422` | The `country` parameter must be a valid [ISO 3166 alpha 2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code. |
| `Batch queries must include 50 queries or less` | `422` | Your batch geocode request cannot contain more than 50 queries. |
| `Query too long {query length}/256 characters` | `422` | Your query cannot contain more than 256 characters. |
| `Query too long - {query tokens length}/20 tokens` | `422` | Your query cannot contain more than 20 word boundary-delimited strings (also known as tokens). |
| `Proximity must be an array in the form [lon, lat]` | `422` | The `proximity` parameter must contain two comma-separated values. |
| `Proximity lon value must be a number between -180 and 180` | `422` | Check the `proximity` parameter's longitude value. |
| `Proximity lat value must be a number between -90 and 90` | `422` | Check the `proximity` parameter's latitude value. |
| `"{input}" is not a valid language code` | `422` | The `language` parameter must be a valid [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) comprised of a mandatory [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) and, optionally, one or more IETF subtags for country or script. |
| `options.language should be a list of no more than 20 languages` | `422` | Your query's `language` parameter cannot contain more than 20 language codes. |
| `options.language should be a list of unique language codes` | `422` | The comma-separated values in `language` must be unique. |
| `limit must be combined with a single type parameter when reverse geocoding` | `422` | If you make a [reverse geocoding request](https://docs.mapbox.com/api/search/geocoding/#reverse-geocoding) with the `limit` parameter, you must also use the `type` parameter. |
| `Rate limit exceeded` | `429` | You have exceeded your set rate limit. Check your [Account page](https://console.mapbox.com/) for more details. |

## Geocoding restrictions and rate limits[](https://docs.mapbox.com/api/search/geocoding/#geocoding-restrictions-and-rate-limits "Direct link to Geocoding restrictions and rate limits")

To protect the Geocoding API and maximize service stability, Mapbox rate limits Geocoding API requests.

- The default Geocoding API rate limit is 1000 requests per minute, but can be adjusted on a per-account basis. Mapbox may automatically adjust customer rate limits to accommodate high traffic applications, or prevent abuse, if deemed appropriate.
- An HTTP error code of `429` will be returned if the rate limit is reached.
- If you require a higher rate limit,[contact us](https://www.mapbox.com/contact/sales/).

## Geocoding API pricing[](https://docs.mapbox.com/api/search/geocoding/#geocoding-api-pricing "Direct link to Geocoding API pricing")

- Billed by **requests**
- See rates and discounts per Geocoding API request in the pricing page's **[Search](https://www.mapbox.com/pricing/#search)** section

Usage of the Geocoding API is measured in **API requests**. Details about the number of requests included in the free tier and the cost per request beyond what is included in the free tier are available on the [pricing page](https://www.mapbox.com/pricing/#search). You may only use responses from the Geocoding API in conjunction with a Mapbox map.

### Autocomplete and pricing[](https://docs.mapbox.com/api/search/geocoding/#autocomplete-and-pricing "Direct link to Autocomplete and pricing")

The Geocoding API's [`autocomplete`](https://docs.mapbox.com/api/search/geocoding/#forward-geocoding-with-search-text-input) parameter is enabled by default. If you are using the API with `autocomplete` enabled, you will likely accumulate one API request per keystroke. For example, if you have autocomplete turned on and you set up your application to make a Geocoding API request every time a user types a character into a search bar, then a user typing "Cali" looking for "California" will be making _four Geocoding API requests_, one for each letter.

To reduce the number of requests made while `autocomplete` is enabled, you can wait to begin making API requests until a specific number of characters are entered. The Geocoding API doesn't have a parameter for limiting requests built in, but you can use the [Mapbox Search JS SDK](https://docs.mapbox.com/mapbox-search-js/guides/) to configure the search configuration, or you can write it into your application directly.

### Batch geocoding and pricing[](https://docs.mapbox.com/api/search/geocoding/#batch-geocoding-and-pricing "Direct link to Batch geocoding and pricing")

[Batch geocoding](https://docs.mapbox.com/api/search/geocoding/#batch-geocoding) allows you to have multiple geocoding queries in a single request. Each individual search in a batch geocoding request counts as one request. For example, a request that contains three searches would count as three requests and be billed as such. These three requests would be reflected on your [statistics graph](https://console.mapbox.com/account/statistics/) and your bill.

## Japan Geocoding[](https://docs.mapbox.com/api/search/geocoding/#japan-geocoding "Direct link to Japan Geocoding")

The Geocoding API includes a robust Japan address and place search. To access the new, high-accuracy and high-coverage Japan search, you must set both the `language` and `country` parameters:

- `language=ja`
- `country=jp`

Japan geocoding responses use the same [geographic feature types section](https://docs.mapbox.com/api/search/geocoding/#geographic-feature-types) as other countries to define a feature's context components (with the addition of a `block` component, where applicable). These feature types represent the following equivalents of a Japanese address structure:

| Geographic feature type | Japan address equivalent |
| ----------------------- | ------------------------ |
| `block`                 | block                    |
| `neighborhood`          | chome                    |
| `locality`              | oaza                     |
| `place`                 | city                     |
| `region`                | prefecture               |

Japan geocoding responses also contain a unique `feature.properties.reading` object for Japanese reading equivalents by script type, for example:

Additional Developer Resources
