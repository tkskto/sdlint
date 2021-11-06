/**
 * @url: https://schema.org/Thing
 */
interface Thing extends StructuredData{
    additionalType?: DataType.Text.URL;
    alternateName?: string;
    description?: string;
    disambiguatingDescription?: string;
    identifier?: Thing.Intangible.StructuredValue.PropertyValue | string | DataType.Text.URL;
    image?: ImageObject | DataType.Text.URL;
    mainEntityOfPage?: Thing.CreativeWork | DataType.Text.URL;
    name?: string;
    potentialAction?: Action;
    sameAs?: DataType.Text.URL;
    subjectOf?: Thing.CreativeWork | Event;
    url?: DataType.Text.URL;
}

declare namespace Thing {
    /**
     * https://schema.org/CreativeWork
     */
    interface CreativeWork extends Thing {

    }

    /**
     * @url: https://schema.org/Place
     */
    interface Place extends Thing {
        additionalProperty?: Thing.Intangible.StructuredValue.PropertyValue;
        address: Thing.Intangible.StructuredValue.ContactPoint.PostalAddress | string;
        aggregateRating: AggregateRating;
        amenityFeature: LocationFeatureSpecification;
        branchCode: string;
        containedInPlace: Thing.Place;
        containsPlace: Thing.Place;
        event: Event;
        faxNumber: string;
        geo: GeoCoordinates | GeoShape;
        geoContains: GeospatialGeometry | Thing.Place;
        geoCoveredBy: GeospatialGeometry | Thing.Place;
        geoCovers: GeospatialGeometry | Thing.Place;
        geoCrosses: GeospatialGeometry | Thing.Place;
        geoDisjoint: GeospatialGeometry | Thing.Place;
        geoIntersects: GeospatialGeometry | Thing.Place;
        geoOverlaps: GeospatialGeometry | Thing.Place;
        geoTouches: GeospatialGeometry | Thing.Place;
        geoWithin: GeospatialGeometry | Thing.Place;
        globalLocationNumber: string;
        hasDriveThroughService: boolean;
        hasMap: Map | DataType.Text.URL;
        isAccessibleForFree: boolean;
        isicV4: string;
        latitude: number | string;
        logo: Thing.CreativeWork.MediaObject.ImageObject | DataType.Text.URL;
        longitude: number | string;
        maximumAttendeeCapacity: number;
        openingHoursSpecification: OpeningHoursSpecification;
        photo: Thing.CreativeWork.MediaObject.ImageObject | Photograph;
        publicAccess: boolean;
        review: Review;
        slogan: string;
        smokingAllowed: boolean;
        specialOpeningHoursSpecification: OpeningHoursSpecification;
        telephone: string;
        tourBookingPage: DataType.Text.URL;
    }

    /**
     * @url: https://schema.org/Organization
     */
    interface Organization extends Thing {

    }

    namespace CreativeWork {

        /**
         * @url: https://schema.org/MediaObject
         */
        interface MediaObject extends CreativeWork {

        }

        namespace MediaObject {
            /**
             * @url: https://schema.org/ImageObject
             */
            interface ImageObject extends MediaObject {
                caption?: Thing.CreativeWork.MediaObject | string;
                exifData?: Thing.Intangible.StructuredValue.PropertyValue | string;
                representativeOfPage?: boolean;
                thumbnail?: Thing.CreativeWork.MediaObject.ImageObject | string;
            }
        }
    }

    namespace Intangible {
        /**
         * @url: https://schema.org/Property
         */
        interface Property extends Thing {
            domainIncludes?: Class;
            inverseOf?: Property;
            rangeIncludes?: Class;
            supersededBy?: Class | Enumeration | Property;
        }
        /**
         * @url: https://schema.org/Class
         */
        interface Class extends Thing {
            supersededBy?: Class | Enumeration | Property;
        }
        /**
         * @url: https://schema.org/Enumeration
         */
        interface Enumeration extends Thing {
            supersededBy?: Class | Enumeration | Property;
        }

        namespace Enumeration {
            /**
             * @url: https://schema.org/QualitativeValue
             */
            interface QualitativeValue extends Thing {}

            /**
             * @url: https://schema.org/QuantitativeValue
             */
            interface QuantitativeValue extends Thing {
                additionalProperty: Thing.Intangible.StructuredValue.PropertyValue;
                maxValue: number;
                minValue: number;
                unitCode: string;
                unitText: DataType.Text.URL;
                value: boolean | number | Thing.Intangible.StructuredValue.PropertyValue | string;
                valueReference: Thing.Intangible.Enumeration | Thing.Intangible.StructuredValue.PropertyValue | Thing.Intangible.Enumeration.QualitativeValue | Thing.Intangible.Enumeration.QuantitativeValue | Thing.Intangible.StructuredValue;
            }
        }

        /**
         * @url: https://schema.org/JobPosting
         */
        interface JobPosting extends Thing {
            applicantLocationRequirements?: AdministrativeArea;
            applicationContact?: ContactPoint;
            baseSalary?: Thing.Intangible.StructuredValue.MonetaryAmount | PriceSpecification | number;
            datePosted?: DataType.DateTime | DataType.Date;
            educationRequirements?: EducationalOccupationalCredential | string;
            eligibilityToWorkRequirement?: string;
            employerOverview?: string;
            employmentType?: string | string[];
            employmentUnit?: Thing.Organization;
            estimatedSalary?: Thing.Intangible.StructuredValue.MonetaryAmount | MonetaryAmountDistribution | number;
            experienceRequirements?: string;
            hiringOrganization?: Thing.Organization;
            incentiveCompensation?: string;
            industry?: DefinedTerm | string;
            jobBenefits?: string;
            jobLocation?: Thing.Place;
            jobLocationType?: string;
            jobStartDate?: DataType.Date | string;
            occupationalCategory?: CategoryCode | string;
            physicalRequirement?: DefinedTerm | DataType.Text.URL | string;
            qualifications?: EducationalOccupationalCredential | string;
            relevantOccupation?: Occupation;
            responsibilities?: string;
            salaryCurrency?: string;
            securityClearanceRequirement?: DataType.Text.URL | string;
            sensoryRequirement?: DefinedTerm | DataType.Text.URL | string;
            skills?: DefinedTerm | string;
            specialCommitments?: string;
            title?: string;
            totalJobOpenings?: number;
            validThrough?: DataType.DateTime | DataType.Date;
            workHours?: string;
        }

        /**
         * @url: https://schema.org/ItemList
         */
        interface ItemList extends Thing {
            itemListElement?: ListItem[] | string[];
            itemListOrder?: ItemListOrderType | string;
            numberOfItems?: number;
        }

        /**
         * @url: https://schema.org/ListItem
         */
        interface ListItem extends Thing {
            '@type': 'ListItem';
            item?: Thing;
            nextItem?: ListItem;
            position?: number | string;
            previousItem?: ListItem
        }

        namespace ItemList {
            /**
             * @url: https://schema.org/BreadcrumbList
             */
            interface BreadcrumbList extends ItemList {}
        }

        namespace StructuredValue {
            /**
             * @url: https://schema.org/PropertyValue
             */
            interface PropertyValue extends Thing {
                maxValue?: number;
                measurementTechnique?: DataType.Text.URL | string;
                minValue?: number;
                propertyID?: DataType.Text.URL | string;
                unitCode?: DataType.Text.URL | string;
                unitText?: string;
                value?: StructuredValue | boolean | number | string;
                valueReference?: Enumeration | PropertyValue | QualitativeValue | QuantitativeValue | StructuredValue;
            }

            interface MonetaryAmount extends Thing {
                currency: string;
                maxValue: number;
                minValue: number;
                validFrom: DataType.Date | DataType.DateTime;
                validThrough: DataType.Date | DataType.DateTime;
                value: boolean | number | Thing.Intangible.StructuredValue | string;
            }

            /**
             * @URL: https://schema.org/ContactPoint
             */
            interface ContactPoint extends Thing {}

            namespace ContactPoint {
                /**
                 * @URL: https://schema.org/PostalAddress
                 */
                interface PostalAddress extends ContactPoint {
                    addressCountry: Country | string;
                    addressLocality: string;
                    addressRegion: string;
                    postOfficeBoxNumber: string;
                    postalCode: string;
                    streetAddress: string;
                }
            }
        }
    }
}
