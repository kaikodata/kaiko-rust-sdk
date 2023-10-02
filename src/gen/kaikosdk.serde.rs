// @generated
impl serde::Serialize for DataInterval {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.DataInterval", len)?;
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataInterval {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_time",
            "startTime",
            "end_time",
            "endTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartTime,
            EndTime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataInterval;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.DataInterval")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DataInterval, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_time__ = None;
                let mut end_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map.next_value()?;
                        }
                    }
                }
                Ok(DataInterval {
                    start_time: start_time__,
                    end_time: end_time__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.DataInterval", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InstrumentCriteria {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.exchange.is_empty() {
            len += 1;
        }
        if !self.instrument_class.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.InstrumentCriteria", len)?;
        if !self.exchange.is_empty() {
            struct_ser.serialize_field("exchange", &self.exchange)?;
        }
        if !self.instrument_class.is_empty() {
            struct_ser.serialize_field("instrumentClass", &self.instrument_class)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstrumentCriteria {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exchange",
            "instrument_class",
            "instrumentClass",
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Exchange,
            InstrumentClass,
            Code,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "exchange" => Ok(GeneratedField::Exchange),
                            "instrumentClass" | "instrument_class" => Ok(GeneratedField::InstrumentClass),
                            "code" => Ok(GeneratedField::Code),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstrumentCriteria;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.InstrumentCriteria")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InstrumentCriteria, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut exchange__ = None;
                let mut instrument_class__ = None;
                let mut code__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Exchange => {
                            if exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exchange"));
                            }
                            exchange__ = Some(map.next_value()?);
                        }
                        GeneratedField::InstrumentClass => {
                            if instrument_class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentClass"));
                            }
                            instrument_class__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(InstrumentCriteria {
                    exchange: exchange__.unwrap_or_default(),
                    instrument_class: instrument_class__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.InstrumentCriteria", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SortCriteria {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::SortUnknown => "SORT_UNKNOWN",
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SortCriteria {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SORT_UNKNOWN",
            "ASC",
            "DESC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortCriteria;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(SortCriteria::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(SortCriteria::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SORT_UNKNOWN" => Ok(SortCriteria::SortUnknown),
                    "ASC" => Ok(SortCriteria::Asc),
                    "DESC" => Ok(SortCriteria::Desc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Source {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.Source", len)?;
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", &self.data)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Source {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
            "price",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
            Price,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "data" => Ok(GeneratedField::Data),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Source;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.Source")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Source, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                let mut price__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Source {
                    data: data__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.Source", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.exchange_code.is_empty() {
            len += 1;
        }
        if self.count != 0 {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        if !self.volume.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.SourceData", len)?;
        if !self.exchange_code.is_empty() {
            struct_ser.serialize_field("exchangeCode", &self.exchange_code)?;
        }
        if self.count != 0 {
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.volume.is_empty() {
            struct_ser.serialize_field("volume", &self.volume)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exchange_code",
            "exchangeCode",
            "count",
            "price",
            "volume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExchangeCode,
            Count,
            Price,
            Volume,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "exchangeCode" | "exchange_code" => Ok(GeneratedField::ExchangeCode),
                            "count" => Ok(GeneratedField::Count),
                            "price" => Ok(GeneratedField::Price),
                            "volume" => Ok(GeneratedField::Volume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.SourceData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SourceData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut exchange_code__ = None;
                let mut count__ = None;
                let mut price__ = None;
                let mut volume__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExchangeCode => {
                            if exchange_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exchangeCode"));
                            }
                            exchange_code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map.next_value()?);
                        }
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SourceData {
                    exchange_code: exchange_code__.unwrap_or_default(),
                    count: count__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    volume: volume__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.SourceData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatedPriceRequestV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.instrument_class.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatedPriceRequestV1", len)?;
        if !self.instrument_class.is_empty() {
            struct_ser.serialize_field("instrumentClass", &self.instrument_class)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatedPriceRequestV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instrument_class",
            "instrumentClass",
            "code",
            "interval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstrumentClass,
            Code,
            Interval,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instrumentClass" | "instrument_class" => Ok(GeneratedField::InstrumentClass),
                            "code" => Ok(GeneratedField::Code),
                            "interval" => Ok(GeneratedField::Interval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatedPriceRequestV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatedPriceRequestV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatedPriceRequestV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instrument_class__ = None;
                let mut code__ = None;
                let mut interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstrumentClass => {
                            if instrument_class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentClass"));
                            }
                            instrument_class__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamAggregatedPriceRequestV1 {
                    instrument_class: instrument_class__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    interval: interval__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatedPriceRequestV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatedPriceResponseV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.aggregate.is_empty() {
            len += 1;
        }
        if !self.instrument_class.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        if self.ts_event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatedPriceResponseV1", len)?;
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        if !self.instrument_class.is_empty() {
            struct_ser.serialize_field("instrumentClass", &self.instrument_class)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if self.event_type != 0 {
            let v = stream_aggregated_price_response_v1::EventType::from_i32(self.event_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if let Some(v) = self.ts_event.as_ref() {
            struct_ser.serialize_field("tsEvent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatedPriceResponseV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aggregate",
            "instrument_class",
            "instrumentClass",
            "code",
            "value",
            "event_type",
            "eventType",
            "ts_event",
            "tsEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Aggregate,
            InstrumentClass,
            Code,
            Value,
            EventType,
            TsEvent,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "instrumentClass" | "instrument_class" => Ok(GeneratedField::InstrumentClass),
                            "code" => Ok(GeneratedField::Code),
                            "value" => Ok(GeneratedField::Value),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "tsEvent" | "ts_event" => Ok(GeneratedField::TsEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatedPriceResponseV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatedPriceResponseV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatedPriceResponseV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut aggregate__ = None;
                let mut instrument_class__ = None;
                let mut code__ = None;
                let mut value__ = None;
                let mut event_type__ = None;
                let mut ts_event__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                        GeneratedField::InstrumentClass => {
                            if instrument_class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentClass"));
                            }
                            instrument_class__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map.next_value::<stream_aggregated_price_response_v1::EventType>()? as i32);
                        }
                        GeneratedField::TsEvent => {
                            if ts_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsEvent"));
                            }
                            ts_event__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamAggregatedPriceResponseV1 {
                    aggregate: aggregate__.unwrap_or_default(),
                    instrument_class: instrument_class__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    value: value__,
                    event_type: event_type__.unwrap_or_default(),
                    ts_event: ts_event__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatedPriceResponseV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_aggregated_price_response_v1::EventType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::BestAsk => "BEST_ASK",
            Self::BestBid => "BEST_BID",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for stream_aggregated_price_response_v1::EventType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "BEST_ASK",
            "BEST_BID",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_aggregated_price_response_v1::EventType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_aggregated_price_response_v1::EventType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_aggregated_price_response_v1::EventType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(stream_aggregated_price_response_v1::EventType::Unknown),
                    "BEST_ASK" => Ok(stream_aggregated_price_response_v1::EventType::BestAsk),
                    "BEST_BID" => Ok(stream_aggregated_price_response_v1::EventType::BestBid),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatedPriceValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price.is_empty() {
            len += 1;
        }
        if !self.volume.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatedPriceValue", len)?;
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.volume.is_empty() {
            struct_ser.serialize_field("volume", &self.volume)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatedPriceValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price",
            "volume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Price,
            Volume,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "price" => Ok(GeneratedField::Price),
                            "volume" => Ok(GeneratedField::Volume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatedPriceValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatedPriceValue")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatedPriceValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price__ = None;
                let mut volume__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map.next_value()?);
                        }
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatedPriceValue {
                    price: price__.unwrap_or_default(),
                    volume: volume__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatedPriceValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatedQuoteRequestV2 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.instrument_class.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        if self.include_unvetted_price {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatedQuoteRequestV2", len)?;
        if !self.instrument_class.is_empty() {
            struct_ser.serialize_field("instrumentClass", &self.instrument_class)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if self.include_unvetted_price {
            struct_ser.serialize_field("includeUnvettedPrice", &self.include_unvetted_price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatedQuoteRequestV2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instrument_class",
            "instrumentClass",
            "code",
            "interval",
            "include_unvetted_price",
            "includeUnvettedPrice",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstrumentClass,
            Code,
            Interval,
            IncludeUnvettedPrice,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instrumentClass" | "instrument_class" => Ok(GeneratedField::InstrumentClass),
                            "code" => Ok(GeneratedField::Code),
                            "interval" => Ok(GeneratedField::Interval),
                            "includeUnvettedPrice" | "include_unvetted_price" => Ok(GeneratedField::IncludeUnvettedPrice),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatedQuoteRequestV2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatedQuoteRequestV2")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatedQuoteRequestV2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instrument_class__ = None;
                let mut code__ = None;
                let mut interval__ = None;
                let mut include_unvetted_price__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstrumentClass => {
                            if instrument_class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentClass"));
                            }
                            instrument_class__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                        GeneratedField::IncludeUnvettedPrice => {
                            if include_unvetted_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeUnvettedPrice"));
                            }
                            include_unvetted_price__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatedQuoteRequestV2 {
                    instrument_class: instrument_class__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    interval: interval__,
                    include_unvetted_price: include_unvetted_price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatedQuoteRequestV2", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatedQuoteResponseV2 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.aggregate.is_empty() {
            len += 1;
        }
        if !self.instrument_class.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        if self.ts_event.is_some() {
            len += 1;
        }
        if self.vetted.is_some() {
            len += 1;
        }
        if self.unvetted.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatedQuoteResponseV2", len)?;
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        if !self.instrument_class.is_empty() {
            struct_ser.serialize_field("instrumentClass", &self.instrument_class)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if self.event_type != 0 {
            let v = stream_aggregated_quote_response_v2::EventType::from_i32(self.event_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if let Some(v) = self.ts_event.as_ref() {
            struct_ser.serialize_field("tsEvent", v)?;
        }
        if let Some(v) = self.vetted.as_ref() {
            struct_ser.serialize_field("vetted", v)?;
        }
        if let Some(v) = self.unvetted.as_ref() {
            struct_ser.serialize_field("unvetted", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatedQuoteResponseV2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aggregate",
            "instrument_class",
            "instrumentClass",
            "code",
            "event_type",
            "eventType",
            "ts_event",
            "tsEvent",
            "vetted",
            "unvetted",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Aggregate,
            InstrumentClass,
            Code,
            EventType,
            TsEvent,
            Vetted,
            Unvetted,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "instrumentClass" | "instrument_class" => Ok(GeneratedField::InstrumentClass),
                            "code" => Ok(GeneratedField::Code),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "tsEvent" | "ts_event" => Ok(GeneratedField::TsEvent),
                            "vetted" => Ok(GeneratedField::Vetted),
                            "unvetted" => Ok(GeneratedField::Unvetted),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatedQuoteResponseV2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatedQuoteResponseV2")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatedQuoteResponseV2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut aggregate__ = None;
                let mut instrument_class__ = None;
                let mut code__ = None;
                let mut event_type__ = None;
                let mut ts_event__ = None;
                let mut vetted__ = None;
                let mut unvetted__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                        GeneratedField::InstrumentClass => {
                            if instrument_class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentClass"));
                            }
                            instrument_class__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map.next_value::<stream_aggregated_quote_response_v2::EventType>()? as i32);
                        }
                        GeneratedField::TsEvent => {
                            if ts_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsEvent"));
                            }
                            ts_event__ = map.next_value()?;
                        }
                        GeneratedField::Vetted => {
                            if vetted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vetted"));
                            }
                            vetted__ = map.next_value()?;
                        }
                        GeneratedField::Unvetted => {
                            if unvetted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unvetted"));
                            }
                            unvetted__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamAggregatedQuoteResponseV2 {
                    aggregate: aggregate__.unwrap_or_default(),
                    instrument_class: instrument_class__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    event_type: event_type__.unwrap_or_default(),
                    ts_event: ts_event__,
                    vetted: vetted__,
                    unvetted: unvetted__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatedQuoteResponseV2", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_aggregated_quote_response_v2::EventType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::BestAsk => "BEST_ASK",
            Self::BestBid => "BEST_BID",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for stream_aggregated_quote_response_v2::EventType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "BEST_ASK",
            "BEST_BID",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_aggregated_quote_response_v2::EventType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_aggregated_quote_response_v2::EventType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_aggregated_quote_response_v2::EventType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(stream_aggregated_quote_response_v2::EventType::Unknown),
                    "BEST_ASK" => Ok(stream_aggregated_quote_response_v2::EventType::BestAsk),
                    "BEST_BID" => Ok(stream_aggregated_quote_response_v2::EventType::BestBid),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatedQuoteValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price.is_empty() {
            len += 1;
        }
        if !self.volume.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatedQuoteValue", len)?;
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.volume.is_empty() {
            struct_ser.serialize_field("volume", &self.volume)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatedQuoteValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price",
            "volume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Price,
            Volume,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "price" => Ok(GeneratedField::Price),
                            "volume" => Ok(GeneratedField::Volume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatedQuoteValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatedQuoteValue")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatedQuoteValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price__ = None;
                let mut volume__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map.next_value()?);
                        }
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatedQuoteValue {
                    price: price__.unwrap_or_default(),
                    volume: volume__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatedQuoteValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatesDirectExchangeRateRequestV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.aggregate.is_empty() {
            len += 1;
        }
        if self.sources {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatesDirectExchangeRateRequestV1", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        if self.sources {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatesDirectExchangeRateRequestV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "aggregate",
            "sources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Aggregate,
            Sources,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "sources" => Ok(GeneratedField::Sources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatesDirectExchangeRateRequestV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatesDirectExchangeRateRequestV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatesDirectExchangeRateRequestV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut aggregate__ = None;
                let mut sources__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatesDirectExchangeRateRequestV1 {
                    code: code__.unwrap_or_default(),
                    aggregate: aggregate__.unwrap_or_default(),
                    sources: sources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatesDirectExchangeRateRequestV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatesDirectExchangeRateResponseV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.aggregate.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        if !self.sequence_id.is_empty() {
            len += 1;
        }
        if !self.sources.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.uid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatesDirectExchangeRateResponseV1", len)?;
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.sequence_id.is_empty() {
            struct_ser.serialize_field("sequenceId", &self.sequence_id)?;
        }
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatesDirectExchangeRateResponseV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aggregate",
            "code",
            "price",
            "sequence_id",
            "sequenceId",
            "sources",
            "timestamp",
            "uid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Aggregate,
            Code,
            Price,
            SequenceId,
            Sources,
            Timestamp,
            Uid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "code" => Ok(GeneratedField::Code),
                            "price" => Ok(GeneratedField::Price),
                            "sequenceId" | "sequence_id" => Ok(GeneratedField::SequenceId),
                            "sources" => Ok(GeneratedField::Sources),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "uid" => Ok(GeneratedField::Uid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatesDirectExchangeRateResponseV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatesDirectExchangeRateResponseV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatesDirectExchangeRateResponseV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut aggregate__ = None;
                let mut code__ = None;
                let mut price__ = None;
                let mut sequence_id__ = None;
                let mut sources__ = None;
                let mut timestamp__ = None;
                let mut uid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map.next_value()?);
                        }
                        GeneratedField::SequenceId => {
                            if sequence_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceId"));
                            }
                            sequence_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatesDirectExchangeRateResponseV1 {
                    aggregate: aggregate__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    sequence_id: sequence_id__.unwrap_or_default(),
                    sources: sources__.unwrap_or_default(),
                    timestamp: timestamp__,
                    uid: uid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatesDirectExchangeRateResponseV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatesOhlcvRequestV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instrument_criteria.is_some() {
            len += 1;
        }
        if !self.aggregate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatesOHLCVRequestV1", len)?;
        if let Some(v) = self.instrument_criteria.as_ref() {
            struct_ser.serialize_field("instrumentCriteria", v)?;
        }
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatesOhlcvRequestV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instrument_criteria",
            "instrumentCriteria",
            "aggregate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstrumentCriteria,
            Aggregate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instrumentCriteria" | "instrument_criteria" => Ok(GeneratedField::InstrumentCriteria),
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatesOhlcvRequestV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatesOHLCVRequestV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatesOhlcvRequestV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instrument_criteria__ = None;
                let mut aggregate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstrumentCriteria => {
                            if instrument_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentCriteria"));
                            }
                            instrument_criteria__ = map.next_value()?;
                        }
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatesOhlcvRequestV1 {
                    instrument_criteria: instrument_criteria__,
                    aggregate: aggregate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatesOHLCVRequestV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatesOhlcvResponseV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.aggregate.is_empty() {
            len += 1;
        }
        if !self.class.is_empty() {
            len += 1;
        }
        if !self.close.is_empty() {
            len += 1;
        }
        if !self.exchange.is_empty() {
            len += 1;
        }
        if !self.high.is_empty() {
            len += 1;
        }
        if !self.low.is_empty() {
            len += 1;
        }
        if !self.open.is_empty() {
            len += 1;
        }
        if !self.sequence_id.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.uid.is_empty() {
            len += 1;
        }
        if !self.volume.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatesOHLCVResponseV1", len)?;
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        if !self.class.is_empty() {
            struct_ser.serialize_field("class", &self.class)?;
        }
        if !self.close.is_empty() {
            struct_ser.serialize_field("close", &self.close)?;
        }
        if !self.exchange.is_empty() {
            struct_ser.serialize_field("exchange", &self.exchange)?;
        }
        if !self.high.is_empty() {
            struct_ser.serialize_field("high", &self.high)?;
        }
        if !self.low.is_empty() {
            struct_ser.serialize_field("low", &self.low)?;
        }
        if !self.open.is_empty() {
            struct_ser.serialize_field("open", &self.open)?;
        }
        if !self.sequence_id.is_empty() {
            struct_ser.serialize_field("sequenceId", &self.sequence_id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if !self.volume.is_empty() {
            struct_ser.serialize_field("volume", &self.volume)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatesOhlcvResponseV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aggregate",
            "class",
            "close",
            "exchange",
            "high",
            "low",
            "open",
            "sequence_id",
            "sequenceId",
            "code",
            "timestamp",
            "uid",
            "volume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Aggregate,
            Class,
            Close,
            Exchange,
            High,
            Low,
            Open,
            SequenceId,
            Code,
            Timestamp,
            Uid,
            Volume,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "class" => Ok(GeneratedField::Class),
                            "close" => Ok(GeneratedField::Close),
                            "exchange" => Ok(GeneratedField::Exchange),
                            "high" => Ok(GeneratedField::High),
                            "low" => Ok(GeneratedField::Low),
                            "open" => Ok(GeneratedField::Open),
                            "sequenceId" | "sequence_id" => Ok(GeneratedField::SequenceId),
                            "code" => Ok(GeneratedField::Code),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "uid" => Ok(GeneratedField::Uid),
                            "volume" => Ok(GeneratedField::Volume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatesOhlcvResponseV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatesOHLCVResponseV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatesOhlcvResponseV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut aggregate__ = None;
                let mut class__ = None;
                let mut close__ = None;
                let mut exchange__ = None;
                let mut high__ = None;
                let mut low__ = None;
                let mut open__ = None;
                let mut sequence_id__ = None;
                let mut code__ = None;
                let mut timestamp__ = None;
                let mut uid__ = None;
                let mut volume__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                        GeneratedField::Class => {
                            if class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            class__ = Some(map.next_value()?);
                        }
                        GeneratedField::Close => {
                            if close__.is_some() {
                                return Err(serde::de::Error::duplicate_field("close"));
                            }
                            close__ = Some(map.next_value()?);
                        }
                        GeneratedField::Exchange => {
                            if exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exchange"));
                            }
                            exchange__ = Some(map.next_value()?);
                        }
                        GeneratedField::High => {
                            if high__.is_some() {
                                return Err(serde::de::Error::duplicate_field("high"));
                            }
                            high__ = Some(map.next_value()?);
                        }
                        GeneratedField::Low => {
                            if low__.is_some() {
                                return Err(serde::de::Error::duplicate_field("low"));
                            }
                            low__ = Some(map.next_value()?);
                        }
                        GeneratedField::Open => {
                            if open__.is_some() {
                                return Err(serde::de::Error::duplicate_field("open"));
                            }
                            open__ = Some(map.next_value()?);
                        }
                        GeneratedField::SequenceId => {
                            if sequence_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceId"));
                            }
                            sequence_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatesOhlcvResponseV1 {
                    aggregate: aggregate__.unwrap_or_default(),
                    class: class__.unwrap_or_default(),
                    close: close__.unwrap_or_default(),
                    exchange: exchange__.unwrap_or_default(),
                    high: high__.unwrap_or_default(),
                    low: low__.unwrap_or_default(),
                    open: open__.unwrap_or_default(),
                    sequence_id: sequence_id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    timestamp: timestamp__,
                    uid: uid__.unwrap_or_default(),
                    volume: volume__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatesOHLCVResponseV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatesSpotExchangeRateRequestV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.aggregate.is_empty() {
            len += 1;
        }
        if self.sources {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatesSpotExchangeRateRequestV1", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        if self.sources {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatesSpotExchangeRateRequestV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "aggregate",
            "sources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Aggregate,
            Sources,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "sources" => Ok(GeneratedField::Sources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatesSpotExchangeRateRequestV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatesSpotExchangeRateRequestV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatesSpotExchangeRateRequestV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut aggregate__ = None;
                let mut sources__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatesSpotExchangeRateRequestV1 {
                    code: code__.unwrap_or_default(),
                    aggregate: aggregate__.unwrap_or_default(),
                    sources: sources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatesSpotExchangeRateRequestV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatesSpotExchangeRateResponseV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.aggregate.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        if !self.sequence_id.is_empty() {
            len += 1;
        }
        if !self.sources.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.uid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatesSpotExchangeRateResponseV1", len)?;
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.sequence_id.is_empty() {
            struct_ser.serialize_field("sequenceId", &self.sequence_id)?;
        }
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatesSpotExchangeRateResponseV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aggregate",
            "code",
            "price",
            "sequence_id",
            "sequenceId",
            "sources",
            "timestamp",
            "uid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Aggregate,
            Code,
            Price,
            SequenceId,
            Sources,
            Timestamp,
            Uid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "code" => Ok(GeneratedField::Code),
                            "price" => Ok(GeneratedField::Price),
                            "sequenceId" | "sequence_id" => Ok(GeneratedField::SequenceId),
                            "sources" => Ok(GeneratedField::Sources),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "uid" => Ok(GeneratedField::Uid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatesSpotExchangeRateResponseV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatesSpotExchangeRateResponseV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatesSpotExchangeRateResponseV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut aggregate__ = None;
                let mut code__ = None;
                let mut price__ = None;
                let mut sequence_id__ = None;
                let mut sources__ = None;
                let mut timestamp__ = None;
                let mut uid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map.next_value()?);
                        }
                        GeneratedField::SequenceId => {
                            if sequence_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceId"));
                            }
                            sequence_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatesSpotExchangeRateResponseV1 {
                    aggregate: aggregate__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    sequence_id: sequence_id__.unwrap_or_default(),
                    sources: sources__.unwrap_or_default(),
                    timestamp: timestamp__,
                    uid: uid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatesSpotExchangeRateResponseV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatesVwapRequestV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instrument_criteria.is_some() {
            len += 1;
        }
        if !self.aggregate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatesVWAPRequestV1", len)?;
        if let Some(v) = self.instrument_criteria.as_ref() {
            struct_ser.serialize_field("instrumentCriteria", v)?;
        }
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatesVwapRequestV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instrument_criteria",
            "instrumentCriteria",
            "aggregate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstrumentCriteria,
            Aggregate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instrumentCriteria" | "instrument_criteria" => Ok(GeneratedField::InstrumentCriteria),
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatesVwapRequestV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatesVWAPRequestV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatesVwapRequestV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instrument_criteria__ = None;
                let mut aggregate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstrumentCriteria => {
                            if instrument_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentCriteria"));
                            }
                            instrument_criteria__ = map.next_value()?;
                        }
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatesVwapRequestV1 {
                    instrument_criteria: instrument_criteria__,
                    aggregate: aggregate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatesVWAPRequestV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamAggregatesVwapResponseV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.aggregate.is_empty() {
            len += 1;
        }
        if !self.class.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.exchange.is_empty() {
            len += 1;
        }
        if !self.sequence_id.is_empty() {
            len += 1;
        }
        if self.price != 0. {
            len += 1;
        }
        if self.ts_event.is_some() {
            len += 1;
        }
        if !self.uid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamAggregatesVWAPResponseV1", len)?;
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        if !self.class.is_empty() {
            struct_ser.serialize_field("class", &self.class)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.exchange.is_empty() {
            struct_ser.serialize_field("exchange", &self.exchange)?;
        }
        if !self.sequence_id.is_empty() {
            struct_ser.serialize_field("sequenceId", &self.sequence_id)?;
        }
        if self.price != 0. {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if let Some(v) = self.ts_event.as_ref() {
            struct_ser.serialize_field("tsEvent", v)?;
        }
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamAggregatesVwapResponseV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aggregate",
            "class",
            "code",
            "exchange",
            "sequence_id",
            "sequenceId",
            "price",
            "ts_event",
            "tsEvent",
            "uid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Aggregate,
            Class,
            Code,
            Exchange,
            SequenceId,
            Price,
            TsEvent,
            Uid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "class" => Ok(GeneratedField::Class),
                            "code" => Ok(GeneratedField::Code),
                            "exchange" => Ok(GeneratedField::Exchange),
                            "sequenceId" | "sequence_id" => Ok(GeneratedField::SequenceId),
                            "price" => Ok(GeneratedField::Price),
                            "tsEvent" | "ts_event" => Ok(GeneratedField::TsEvent),
                            "uid" => Ok(GeneratedField::Uid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamAggregatesVwapResponseV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamAggregatesVWAPResponseV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamAggregatesVwapResponseV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut aggregate__ = None;
                let mut class__ = None;
                let mut code__ = None;
                let mut exchange__ = None;
                let mut sequence_id__ = None;
                let mut price__ = None;
                let mut ts_event__ = None;
                let mut uid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Aggregate => {
                            if aggregate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate__ = Some(map.next_value()?);
                        }
                        GeneratedField::Class => {
                            if class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            class__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Exchange => {
                            if exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exchange"));
                            }
                            exchange__ = Some(map.next_value()?);
                        }
                        GeneratedField::SequenceId => {
                            if sequence_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceId"));
                            }
                            sequence_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TsEvent => {
                            if ts_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsEvent"));
                            }
                            ts_event__ = map.next_value()?;
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamAggregatesVwapResponseV1 {
                    aggregate: aggregate__.unwrap_or_default(),
                    class: class__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    exchange: exchange__.unwrap_or_default(),
                    sequence_id: sequence_id__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    ts_event: ts_event__,
                    uid: uid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamAggregatesVWAPResponseV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexCommodity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::SicUnknown => "SIC_UNKNOWN",
            Self::SicRealTime => "SIC_REAL_TIME",
            Self::SicDailyFixing => "SIC_DAILY_FIXING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexCommodity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIC_UNKNOWN",
            "SIC_REAL_TIME",
            "SIC_DAILY_FIXING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexCommodity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(StreamIndexCommodity::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(StreamIndexCommodity::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SIC_UNKNOWN" => Ok(StreamIndexCommodity::SicUnknown),
                    "SIC_REAL_TIME" => Ok(StreamIndexCommodity::SicRealTime),
                    "SIC_DAILY_FIXING" => Ok(StreamIndexCommodity::SicDailyFixing),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexMultiAssetsServiceRequestV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.index_code.is_empty() {
            len += 1;
        }
        if !self.commodities.is_empty() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexMultiAssetsServiceRequestV1", len)?;
        if !self.index_code.is_empty() {
            struct_ser.serialize_field("indexCode", &self.index_code)?;
        }
        if !self.commodities.is_empty() {
            let v = self.commodities.iter().cloned().map(|v| {
                StreamIndexCommodity::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("commodities", &v)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexMultiAssetsServiceRequestV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index_code",
            "indexCode",
            "commodities",
            "interval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IndexCode,
            Commodities,
            Interval,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "indexCode" | "index_code" => Ok(GeneratedField::IndexCode),
                            "commodities" => Ok(GeneratedField::Commodities),
                            "interval" => Ok(GeneratedField::Interval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexMultiAssetsServiceRequestV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexMultiAssetsServiceRequestV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexMultiAssetsServiceRequestV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut index_code__ = None;
                let mut commodities__ = None;
                let mut interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IndexCode => {
                            if index_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexCode"));
                            }
                            index_code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Commodities => {
                            if commodities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commodities"));
                            }
                            commodities__ = Some(map.next_value::<Vec<StreamIndexCommodity>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamIndexMultiAssetsServiceRequestV1 {
                    index_code: index_code__.unwrap_or_default(),
                    commodities: commodities__.unwrap_or_default(),
                    interval: interval__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexMultiAssetsServiceRequestV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexMultiAssetsServiceResponseComposition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.underlying_instrument.is_empty() {
            len += 1;
        }
        if !self.base.is_empty() {
            len += 1;
        }
        if !self.quote.is_empty() {
            len += 1;
        }
        if !self.exchanges.is_empty() {
            len += 1;
        }
        if self.instrument_interval.is_some() {
            len += 1;
        }
        if !self.currency_conversion.is_empty() {
            len += 1;
        }
        if self.ts_event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexMultiAssetsServiceResponseComposition", len)?;
        if !self.underlying_instrument.is_empty() {
            struct_ser.serialize_field("underlyingInstrument", &self.underlying_instrument)?;
        }
        if !self.base.is_empty() {
            struct_ser.serialize_field("base", &self.base)?;
        }
        if !self.quote.is_empty() {
            struct_ser.serialize_field("quote", &self.quote)?;
        }
        if !self.exchanges.is_empty() {
            struct_ser.serialize_field("exchanges", &self.exchanges)?;
        }
        if let Some(v) = self.instrument_interval.as_ref() {
            struct_ser.serialize_field("instrumentInterval", v)?;
        }
        if !self.currency_conversion.is_empty() {
            struct_ser.serialize_field("currencyConversion", &self.currency_conversion)?;
        }
        if let Some(v) = self.ts_event.as_ref() {
            struct_ser.serialize_field("tsEvent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexMultiAssetsServiceResponseComposition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "underlying_instrument",
            "underlyingInstrument",
            "base",
            "quote",
            "exchanges",
            "instrument_interval",
            "instrumentInterval",
            "currency_conversion",
            "currencyConversion",
            "ts_event",
            "tsEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnderlyingInstrument,
            Base,
            Quote,
            Exchanges,
            InstrumentInterval,
            CurrencyConversion,
            TsEvent,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "underlyingInstrument" | "underlying_instrument" => Ok(GeneratedField::UnderlyingInstrument),
                            "base" => Ok(GeneratedField::Base),
                            "quote" => Ok(GeneratedField::Quote),
                            "exchanges" => Ok(GeneratedField::Exchanges),
                            "instrumentInterval" | "instrument_interval" => Ok(GeneratedField::InstrumentInterval),
                            "currencyConversion" | "currency_conversion" => Ok(GeneratedField::CurrencyConversion),
                            "tsEvent" | "ts_event" => Ok(GeneratedField::TsEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexMultiAssetsServiceResponseComposition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexMultiAssetsServiceResponseComposition")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexMultiAssetsServiceResponseComposition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut underlying_instrument__ = None;
                let mut base__ = None;
                let mut quote__ = None;
                let mut exchanges__ = None;
                let mut instrument_interval__ = None;
                let mut currency_conversion__ = None;
                let mut ts_event__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UnderlyingInstrument => {
                            if underlying_instrument__.is_some() {
                                return Err(serde::de::Error::duplicate_field("underlyingInstrument"));
                            }
                            underlying_instrument__ = Some(map.next_value()?);
                        }
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = Some(map.next_value()?);
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = Some(map.next_value()?);
                        }
                        GeneratedField::Exchanges => {
                            if exchanges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exchanges"));
                            }
                            exchanges__ = Some(map.next_value()?);
                        }
                        GeneratedField::InstrumentInterval => {
                            if instrument_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentInterval"));
                            }
                            instrument_interval__ = map.next_value()?;
                        }
                        GeneratedField::CurrencyConversion => {
                            if currency_conversion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currencyConversion"));
                            }
                            currency_conversion__ = Some(map.next_value()?);
                        }
                        GeneratedField::TsEvent => {
                            if ts_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsEvent"));
                            }
                            ts_event__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamIndexMultiAssetsServiceResponseComposition {
                    underlying_instrument: underlying_instrument__.unwrap_or_default(),
                    base: base__.unwrap_or_default(),
                    quote: quote__.unwrap_or_default(),
                    exchanges: exchanges__.unwrap_or_default(),
                    instrument_interval: instrument_interval__,
                    currency_conversion: currency_conversion__.unwrap_or_default(),
                    ts_event: ts_event__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexMultiAssetsServiceResponseComposition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexMultiAssetsServiceResponsePair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.underlying_instrument.is_empty() {
            len += 1;
        }
        if self.underlying_price.is_some() {
            len += 1;
        }
        if self.weighting_factor != 0. {
            len += 1;
        }
        if self.capping_factor != 0. {
            len += 1;
        }
        if self.currency_conversion_factor != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexMultiAssetsServiceResponsePair", len)?;
        if !self.underlying_instrument.is_empty() {
            struct_ser.serialize_field("underlyingInstrument", &self.underlying_instrument)?;
        }
        if let Some(v) = self.underlying_price.as_ref() {
            struct_ser.serialize_field("underlyingPrice", v)?;
        }
        if self.weighting_factor != 0. {
            struct_ser.serialize_field("weightingFactor", &self.weighting_factor)?;
        }
        if self.capping_factor != 0. {
            struct_ser.serialize_field("cappingFactor", &self.capping_factor)?;
        }
        if self.currency_conversion_factor != 0. {
            struct_ser.serialize_field("currencyConversionFactor", &self.currency_conversion_factor)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexMultiAssetsServiceResponsePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "underlying_instrument",
            "underlyingInstrument",
            "underlying_price",
            "underlyingPrice",
            "weighting_factor",
            "weightingFactor",
            "capping_factor",
            "cappingFactor",
            "currency_conversion_factor",
            "currencyConversionFactor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnderlyingInstrument,
            UnderlyingPrice,
            WeightingFactor,
            CappingFactor,
            CurrencyConversionFactor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "underlyingInstrument" | "underlying_instrument" => Ok(GeneratedField::UnderlyingInstrument),
                            "underlyingPrice" | "underlying_price" => Ok(GeneratedField::UnderlyingPrice),
                            "weightingFactor" | "weighting_factor" => Ok(GeneratedField::WeightingFactor),
                            "cappingFactor" | "capping_factor" => Ok(GeneratedField::CappingFactor),
                            "currencyConversionFactor" | "currency_conversion_factor" => Ok(GeneratedField::CurrencyConversionFactor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexMultiAssetsServiceResponsePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexMultiAssetsServiceResponsePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexMultiAssetsServiceResponsePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut underlying_instrument__ = None;
                let mut underlying_price__ = None;
                let mut weighting_factor__ = None;
                let mut capping_factor__ = None;
                let mut currency_conversion_factor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UnderlyingInstrument => {
                            if underlying_instrument__.is_some() {
                                return Err(serde::de::Error::duplicate_field("underlyingInstrument"));
                            }
                            underlying_instrument__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnderlyingPrice => {
                            if underlying_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("underlyingPrice"));
                            }
                            underlying_price__ = map.next_value()?;
                        }
                        GeneratedField::WeightingFactor => {
                            if weighting_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weightingFactor"));
                            }
                            weighting_factor__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CappingFactor => {
                            if capping_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cappingFactor"));
                            }
                            capping_factor__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CurrencyConversionFactor => {
                            if currency_conversion_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currencyConversionFactor"));
                            }
                            currency_conversion_factor__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StreamIndexMultiAssetsServiceResponsePair {
                    underlying_instrument: underlying_instrument__.unwrap_or_default(),
                    underlying_price: underlying_price__,
                    weighting_factor: weighting_factor__.unwrap_or_default(),
                    capping_factor: capping_factor__.unwrap_or_default(),
                    currency_conversion_factor: currency_conversion_factor__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexMultiAssetsServiceResponsePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexMultiAssetsServiceResponsePrices {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.index_value != 0. {
            len += 1;
        }
        if self.divisor != 0. {
            len += 1;
        }
        if !self.pairs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexMultiAssetsServiceResponsePrices", len)?;
        if self.index_value != 0. {
            struct_ser.serialize_field("indexValue", &self.index_value)?;
        }
        if self.divisor != 0. {
            struct_ser.serialize_field("divisor", &self.divisor)?;
        }
        if !self.pairs.is_empty() {
            struct_ser.serialize_field("pairs", &self.pairs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexMultiAssetsServiceResponsePrices {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index_value",
            "indexValue",
            "divisor",
            "pairs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IndexValue,
            Divisor,
            Pairs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "indexValue" | "index_value" => Ok(GeneratedField::IndexValue),
                            "divisor" => Ok(GeneratedField::Divisor),
                            "pairs" => Ok(GeneratedField::Pairs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexMultiAssetsServiceResponsePrices;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexMultiAssetsServiceResponsePrices")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexMultiAssetsServiceResponsePrices, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut index_value__ = None;
                let mut divisor__ = None;
                let mut pairs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IndexValue => {
                            if index_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexValue"));
                            }
                            index_value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Divisor => {
                            if divisor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("divisor"));
                            }
                            divisor__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pairs => {
                            if pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pairs"));
                            }
                            pairs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamIndexMultiAssetsServiceResponsePrices {
                    index_value: index_value__.unwrap_or_default(),
                    divisor: divisor__.unwrap_or_default(),
                    pairs: pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexMultiAssetsServiceResponsePrices", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexMultiAssetsServiceResponseV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.commodity != 0 {
            len += 1;
        }
        if !self.index_code.is_empty() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        if !self.main_quote.is_empty() {
            len += 1;
        }
        if !self.compositions.is_empty() {
            len += 1;
        }
        if self.price.is_some() {
            len += 1;
        }
        if self.ts_event.is_some() {
            len += 1;
        }
        if self.ts_compute.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexMultiAssetsServiceResponseV1", len)?;
        if self.commodity != 0 {
            let v = StreamIndexCommodity::from_i32(self.commodity)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.commodity)))?;
            struct_ser.serialize_field("commodity", &v)?;
        }
        if !self.index_code.is_empty() {
            struct_ser.serialize_field("indexCode", &self.index_code)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if !self.main_quote.is_empty() {
            struct_ser.serialize_field("mainQuote", &self.main_quote)?;
        }
        if !self.compositions.is_empty() {
            struct_ser.serialize_field("compositions", &self.compositions)?;
        }
        if let Some(v) = self.price.as_ref() {
            struct_ser.serialize_field("price", v)?;
        }
        if let Some(v) = self.ts_event.as_ref() {
            struct_ser.serialize_field("tsEvent", v)?;
        }
        if let Some(v) = self.ts_compute.as_ref() {
            struct_ser.serialize_field("tsCompute", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexMultiAssetsServiceResponseV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commodity",
            "index_code",
            "indexCode",
            "interval",
            "main_quote",
            "mainQuote",
            "compositions",
            "price",
            "ts_event",
            "tsEvent",
            "ts_compute",
            "tsCompute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Commodity,
            IndexCode,
            Interval,
            MainQuote,
            Compositions,
            Price,
            TsEvent,
            TsCompute,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "commodity" => Ok(GeneratedField::Commodity),
                            "indexCode" | "index_code" => Ok(GeneratedField::IndexCode),
                            "interval" => Ok(GeneratedField::Interval),
                            "mainQuote" | "main_quote" => Ok(GeneratedField::MainQuote),
                            "compositions" => Ok(GeneratedField::Compositions),
                            "price" => Ok(GeneratedField::Price),
                            "tsEvent" | "ts_event" => Ok(GeneratedField::TsEvent),
                            "tsCompute" | "ts_compute" => Ok(GeneratedField::TsCompute),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexMultiAssetsServiceResponseV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexMultiAssetsServiceResponseV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexMultiAssetsServiceResponseV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut commodity__ = None;
                let mut index_code__ = None;
                let mut interval__ = None;
                let mut main_quote__ = None;
                let mut compositions__ = None;
                let mut price__ = None;
                let mut ts_event__ = None;
                let mut ts_compute__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Commodity => {
                            if commodity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commodity"));
                            }
                            commodity__ = Some(map.next_value::<StreamIndexCommodity>()? as i32);
                        }
                        GeneratedField::IndexCode => {
                            if index_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexCode"));
                            }
                            index_code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                        GeneratedField::MainQuote => {
                            if main_quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mainQuote"));
                            }
                            main_quote__ = Some(map.next_value()?);
                        }
                        GeneratedField::Compositions => {
                            if compositions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compositions"));
                            }
                            compositions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = map.next_value()?;
                        }
                        GeneratedField::TsEvent => {
                            if ts_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsEvent"));
                            }
                            ts_event__ = map.next_value()?;
                        }
                        GeneratedField::TsCompute => {
                            if ts_compute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsCompute"));
                            }
                            ts_compute__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamIndexMultiAssetsServiceResponseV1 {
                    commodity: commodity__.unwrap_or_default(),
                    index_code: index_code__.unwrap_or_default(),
                    interval: interval__,
                    main_quote: main_quote__.unwrap_or_default(),
                    compositions: compositions__.unwrap_or_default(),
                    price: price__,
                    ts_event: ts_event__,
                    ts_compute: ts_compute__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexMultiAssetsServiceResponseV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexServiceRequestV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.index_code.is_empty() {
            len += 1;
        }
        if !self.commodities.is_empty() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexServiceRequestV1", len)?;
        if !self.index_code.is_empty() {
            struct_ser.serialize_field("indexCode", &self.index_code)?;
        }
        if !self.commodities.is_empty() {
            let v = self.commodities.iter().cloned().map(|v| {
                StreamIndexCommodity::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("commodities", &v)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexServiceRequestV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index_code",
            "indexCode",
            "commodities",
            "interval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IndexCode,
            Commodities,
            Interval,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "indexCode" | "index_code" => Ok(GeneratedField::IndexCode),
                            "commodities" => Ok(GeneratedField::Commodities),
                            "interval" => Ok(GeneratedField::Interval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexServiceRequestV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexServiceRequestV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexServiceRequestV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut index_code__ = None;
                let mut commodities__ = None;
                let mut interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IndexCode => {
                            if index_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexCode"));
                            }
                            index_code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Commodities => {
                            if commodities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commodities"));
                            }
                            commodities__ = Some(map.next_value::<Vec<StreamIndexCommodity>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamIndexServiceRequestV1 {
                    index_code: index_code__.unwrap_or_default(),
                    commodities: commodities__.unwrap_or_default(),
                    interval: interval__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexServiceRequestV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexServiceResponseBaseAsset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset.is_empty() {
            len += 1;
        }
        if self.weight != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexServiceResponseBaseAsset", len)?;
        if !self.asset.is_empty() {
            struct_ser.serialize_field("asset", &self.asset)?;
        }
        if self.weight != 0. {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexServiceResponseBaseAsset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset",
            "weight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Asset,
            Weight,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "asset" => Ok(GeneratedField::Asset),
                            "weight" => Ok(GeneratedField::Weight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexServiceResponseBaseAsset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexServiceResponseBaseAsset")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexServiceResponseBaseAsset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset__ = None;
                let mut weight__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Asset => {
                            if asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asset"));
                            }
                            asset__ = Some(map.next_value()?);
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StreamIndexServiceResponseBaseAsset {
                    asset: asset__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexServiceResponseBaseAsset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexServiceResponseInstruments {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.partition.is_empty() {
            len += 1;
        }
        if self.price != 0. {
            len += 1;
        }
        if self.volume != 0. {
            len += 1;
        }
        if self.count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexServiceResponseInstruments", len)?;
        if !self.partition.is_empty() {
            struct_ser.serialize_field("partition", &self.partition)?;
        }
        if self.price != 0. {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if self.volume != 0. {
            struct_ser.serialize_field("volume", &self.volume)?;
        }
        if self.count != 0 {
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexServiceResponseInstruments {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "partition",
            "price",
            "volume",
            "count",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Partition,
            Price,
            Volume,
            Count,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "partition" => Ok(GeneratedField::Partition),
                            "price" => Ok(GeneratedField::Price),
                            "volume" => Ok(GeneratedField::Volume),
                            "count" => Ok(GeneratedField::Count),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexServiceResponseInstruments;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexServiceResponseInstruments")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexServiceResponseInstruments, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut partition__ = None;
                let mut price__ = None;
                let mut volume__ = None;
                let mut count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Partition => {
                            if partition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partition"));
                            }
                            partition__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StreamIndexServiceResponseInstruments {
                    partition: partition__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    volume: volume__.unwrap_or_default(),
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexServiceResponseInstruments", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexServiceResponsePairs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pair.is_empty() {
            len += 1;
        }
        if self.weight != 0. {
            len += 1;
        }
        if !self.instruments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexServiceResponsePairs", len)?;
        if !self.pair.is_empty() {
            struct_ser.serialize_field("pair", &self.pair)?;
        }
        if self.weight != 0. {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if !self.instruments.is_empty() {
            struct_ser.serialize_field("instruments", &self.instruments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexServiceResponsePairs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pair",
            "weight",
            "instruments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pair,
            Weight,
            Instruments,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pair" => Ok(GeneratedField::Pair),
                            "weight" => Ok(GeneratedField::Weight),
                            "instruments" => Ok(GeneratedField::Instruments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexServiceResponsePairs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexServiceResponsePairs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexServiceResponsePairs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pair__ = None;
                let mut weight__ = None;
                let mut instruments__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pair => {
                            if pair__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pair"));
                            }
                            pair__ = Some(map.next_value()?);
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Instruments => {
                            if instruments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instruments"));
                            }
                            instruments__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamIndexServiceResponsePairs {
                    pair: pair__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                    instruments: instruments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexServiceResponsePairs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexServiceResponsePercentage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.percentage != 0. {
            len += 1;
        }
        if self.price != 0. {
            len += 1;
        }
        if !self.pairs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexServiceResponsePercentage", len)?;
        if self.percentage != 0. {
            struct_ser.serialize_field("percentage", &self.percentage)?;
        }
        if self.price != 0. {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.pairs.is_empty() {
            struct_ser.serialize_field("pairs", &self.pairs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexServiceResponsePercentage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "percentage",
            "price",
            "pairs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Percentage,
            Price,
            Pairs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "percentage" => Ok(GeneratedField::Percentage),
                            "price" => Ok(GeneratedField::Price),
                            "pairs" => Ok(GeneratedField::Pairs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexServiceResponsePercentage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexServiceResponsePercentage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexServiceResponsePercentage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut percentage__ = None;
                let mut price__ = None;
                let mut pairs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Percentage => {
                            if percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("percentage"));
                            }
                            percentage__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pairs => {
                            if pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pairs"));
                            }
                            pairs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(StreamIndexServiceResponsePercentage {
                    percentage: percentage__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    pairs: pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexServiceResponsePercentage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamIndexServiceResponseV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.index_code.is_empty() {
            len += 1;
        }
        if self.commodity != 0 {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        if !self.quote.is_empty() {
            len += 1;
        }
        if !self.bases.is_empty() {
            len += 1;
        }
        if !self.exchanges.is_empty() {
            len += 1;
        }
        if !self.percentages.is_empty() {
            len += 1;
        }
        if self.ts_event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamIndexServiceResponseV1", len)?;
        if !self.index_code.is_empty() {
            struct_ser.serialize_field("indexCode", &self.index_code)?;
        }
        if self.commodity != 0 {
            let v = StreamIndexCommodity::from_i32(self.commodity)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.commodity)))?;
            struct_ser.serialize_field("commodity", &v)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if !self.quote.is_empty() {
            struct_ser.serialize_field("quote", &self.quote)?;
        }
        if !self.bases.is_empty() {
            struct_ser.serialize_field("bases", &self.bases)?;
        }
        if !self.exchanges.is_empty() {
            struct_ser.serialize_field("exchanges", &self.exchanges)?;
        }
        if !self.percentages.is_empty() {
            struct_ser.serialize_field("percentages", &self.percentages)?;
        }
        if let Some(v) = self.ts_event.as_ref() {
            struct_ser.serialize_field("tsEvent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamIndexServiceResponseV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index_code",
            "indexCode",
            "commodity",
            "interval",
            "quote",
            "bases",
            "exchanges",
            "percentages",
            "ts_event",
            "tsEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IndexCode,
            Commodity,
            Interval,
            Quote,
            Bases,
            Exchanges,
            Percentages,
            TsEvent,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "indexCode" | "index_code" => Ok(GeneratedField::IndexCode),
                            "commodity" => Ok(GeneratedField::Commodity),
                            "interval" => Ok(GeneratedField::Interval),
                            "quote" => Ok(GeneratedField::Quote),
                            "bases" => Ok(GeneratedField::Bases),
                            "exchanges" => Ok(GeneratedField::Exchanges),
                            "percentages" => Ok(GeneratedField::Percentages),
                            "tsEvent" | "ts_event" => Ok(GeneratedField::TsEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamIndexServiceResponseV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamIndexServiceResponseV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamIndexServiceResponseV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut index_code__ = None;
                let mut commodity__ = None;
                let mut interval__ = None;
                let mut quote__ = None;
                let mut bases__ = None;
                let mut exchanges__ = None;
                let mut percentages__ = None;
                let mut ts_event__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IndexCode => {
                            if index_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexCode"));
                            }
                            index_code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Commodity => {
                            if commodity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commodity"));
                            }
                            commodity__ = Some(map.next_value::<StreamIndexCommodity>()? as i32);
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = Some(map.next_value()?);
                        }
                        GeneratedField::Bases => {
                            if bases__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bases"));
                            }
                            bases__ = Some(map.next_value()?);
                        }
                        GeneratedField::Exchanges => {
                            if exchanges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exchanges"));
                            }
                            exchanges__ = Some(map.next_value()?);
                        }
                        GeneratedField::Percentages => {
                            if percentages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("percentages"));
                            }
                            percentages__ = Some(map.next_value()?);
                        }
                        GeneratedField::TsEvent => {
                            if ts_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsEvent"));
                            }
                            ts_event__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamIndexServiceResponseV1 {
                    index_code: index_code__.unwrap_or_default(),
                    commodity: commodity__.unwrap_or_default(),
                    interval: interval__,
                    quote: quote__.unwrap_or_default(),
                    bases: bases__.unwrap_or_default(),
                    exchanges: exchanges__.unwrap_or_default(),
                    percentages: percentages__.unwrap_or_default(),
                    ts_event: ts_event__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamIndexServiceResponseV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamMarketUpdateCommodity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::SmucUnknown => "SMUC_UNKNOWN",
            Self::SmucTrade => "SMUC_TRADE",
            Self::SmucTopOfBook => "SMUC_TOP_OF_BOOK",
            Self::SmucFullOrderBook => "SMUC_FULL_ORDER_BOOK",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for StreamMarketUpdateCommodity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SMUC_UNKNOWN",
            "SMUC_TRADE",
            "SMUC_TOP_OF_BOOK",
            "SMUC_FULL_ORDER_BOOK",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamMarketUpdateCommodity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(StreamMarketUpdateCommodity::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(StreamMarketUpdateCommodity::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SMUC_UNKNOWN" => Ok(StreamMarketUpdateCommodity::SmucUnknown),
                    "SMUC_TRADE" => Ok(StreamMarketUpdateCommodity::SmucTrade),
                    "SMUC_TOP_OF_BOOK" => Ok(StreamMarketUpdateCommodity::SmucTopOfBook),
                    "SMUC_FULL_ORDER_BOOK" => Ok(StreamMarketUpdateCommodity::SmucFullOrderBook),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamMarketUpdateRequestV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instrument_criteria.is_some() {
            len += 1;
        }
        if !self.commodities.is_empty() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        if self.snapshot_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamMarketUpdateRequestV1", len)?;
        if let Some(v) = self.instrument_criteria.as_ref() {
            struct_ser.serialize_field("instrumentCriteria", v)?;
        }
        if !self.commodities.is_empty() {
            let v = self.commodities.iter().cloned().map(|v| {
                StreamMarketUpdateCommodity::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("commodities", &v)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if self.snapshot_type != 0 {
            let v = stream_market_update_request_v1::OrderbookSnapshotType::from_i32(self.snapshot_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.snapshot_type)))?;
            struct_ser.serialize_field("snapshotType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamMarketUpdateRequestV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instrument_criteria",
            "instrumentCriteria",
            "commodities",
            "interval",
            "snapshot_type",
            "snapshotType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstrumentCriteria,
            Commodities,
            Interval,
            SnapshotType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instrumentCriteria" | "instrument_criteria" => Ok(GeneratedField::InstrumentCriteria),
                            "commodities" => Ok(GeneratedField::Commodities),
                            "interval" => Ok(GeneratedField::Interval),
                            "snapshotType" | "snapshot_type" => Ok(GeneratedField::SnapshotType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamMarketUpdateRequestV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamMarketUpdateRequestV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamMarketUpdateRequestV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instrument_criteria__ = None;
                let mut commodities__ = None;
                let mut interval__ = None;
                let mut snapshot_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstrumentCriteria => {
                            if instrument_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentCriteria"));
                            }
                            instrument_criteria__ = map.next_value()?;
                        }
                        GeneratedField::Commodities => {
                            if commodities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commodities"));
                            }
                            commodities__ = Some(map.next_value::<Vec<StreamMarketUpdateCommodity>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                        GeneratedField::SnapshotType => {
                            if snapshot_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshotType"));
                            }
                            snapshot_type__ = Some(map.next_value::<stream_market_update_request_v1::OrderbookSnapshotType>()? as i32);
                        }
                    }
                }
                Ok(StreamMarketUpdateRequestV1 {
                    instrument_criteria: instrument_criteria__,
                    commodities: commodities__.unwrap_or_default(),
                    interval: interval__,
                    snapshot_type: snapshot_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamMarketUpdateRequestV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_market_update_request_v1::OrderbookSnapshotType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::TenPercent => "TEN_PERCENT",
            Self::Full => "FULL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for stream_market_update_request_v1::OrderbookSnapshotType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "TEN_PERCENT",
            "FULL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_market_update_request_v1::OrderbookSnapshotType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_market_update_request_v1::OrderbookSnapshotType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_market_update_request_v1::OrderbookSnapshotType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(stream_market_update_request_v1::OrderbookSnapshotType::Unknown),
                    "TEN_PERCENT" => Ok(stream_market_update_request_v1::OrderbookSnapshotType::TenPercent),
                    "FULL" => Ok(stream_market_update_request_v1::OrderbookSnapshotType::Full),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamMarketUpdateResponseV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.commodity != 0 {
            len += 1;
        }
        if self.amount != 0. {
            len += 1;
        }
        if !self.class.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.exchange.is_empty() {
            len += 1;
        }
        if !self.sequence_id.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if self.price != 0. {
            len += 1;
        }
        if self.ts_exchange.is_some() {
            len += 1;
        }
        if self.ts_collection.is_some() {
            len += 1;
        }
        if self.ts_event.is_some() {
            len += 1;
        }
        if self.update_type != 0 {
            len += 1;
        }
        if self.snapshot.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamMarketUpdateResponseV1", len)?;
        if self.commodity != 0 {
            let v = StreamMarketUpdateCommodity::from_i32(self.commodity)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.commodity)))?;
            struct_ser.serialize_field("commodity", &v)?;
        }
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.class.is_empty() {
            struct_ser.serialize_field("class", &self.class)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.exchange.is_empty() {
            struct_ser.serialize_field("exchange", &self.exchange)?;
        }
        if !self.sequence_id.is_empty() {
            struct_ser.serialize_field("sequenceId", &self.sequence_id)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.price != 0. {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if let Some(v) = self.ts_exchange.as_ref() {
            struct_ser.serialize_field("tsExchange", v)?;
        }
        if let Some(v) = self.ts_collection.as_ref() {
            struct_ser.serialize_field("tsCollection", v)?;
        }
        if let Some(v) = self.ts_event.as_ref() {
            struct_ser.serialize_field("tsEvent", v)?;
        }
        if self.update_type != 0 {
            let v = stream_market_update_response_v1::StreamMarketUpdateType::from_i32(self.update_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.update_type)))?;
            struct_ser.serialize_field("updateType", &v)?;
        }
        if let Some(v) = self.snapshot.as_ref() {
            struct_ser.serialize_field("snapshot", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamMarketUpdateResponseV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commodity",
            "amount",
            "class",
            "code",
            "exchange",
            "sequence_id",
            "sequenceId",
            "id",
            "price",
            "ts_exchange",
            "tsExchange",
            "ts_collection",
            "tsCollection",
            "ts_event",
            "tsEvent",
            "update_type",
            "updateType",
            "snapshot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Commodity,
            Amount,
            Class,
            Code,
            Exchange,
            SequenceId,
            Id,
            Price,
            TsExchange,
            TsCollection,
            TsEvent,
            UpdateType,
            Snapshot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "commodity" => Ok(GeneratedField::Commodity),
                            "amount" => Ok(GeneratedField::Amount),
                            "class" => Ok(GeneratedField::Class),
                            "code" => Ok(GeneratedField::Code),
                            "exchange" => Ok(GeneratedField::Exchange),
                            "sequenceId" | "sequence_id" => Ok(GeneratedField::SequenceId),
                            "id" => Ok(GeneratedField::Id),
                            "price" => Ok(GeneratedField::Price),
                            "tsExchange" | "ts_exchange" => Ok(GeneratedField::TsExchange),
                            "tsCollection" | "ts_collection" => Ok(GeneratedField::TsCollection),
                            "tsEvent" | "ts_event" => Ok(GeneratedField::TsEvent),
                            "updateType" | "update_type" => Ok(GeneratedField::UpdateType),
                            "snapshot" => Ok(GeneratedField::Snapshot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamMarketUpdateResponseV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamMarketUpdateResponseV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamMarketUpdateResponseV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut commodity__ = None;
                let mut amount__ = None;
                let mut class__ = None;
                let mut code__ = None;
                let mut exchange__ = None;
                let mut sequence_id__ = None;
                let mut id__ = None;
                let mut price__ = None;
                let mut ts_exchange__ = None;
                let mut ts_collection__ = None;
                let mut ts_event__ = None;
                let mut update_type__ = None;
                let mut snapshot__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Commodity => {
                            if commodity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commodity"));
                            }
                            commodity__ = Some(map.next_value::<StreamMarketUpdateCommodity>()? as i32);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Class => {
                            if class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            class__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Exchange => {
                            if exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exchange"));
                            }
                            exchange__ = Some(map.next_value()?);
                        }
                        GeneratedField::SequenceId => {
                            if sequence_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceId"));
                            }
                            sequence_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TsExchange => {
                            if ts_exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsExchange"));
                            }
                            ts_exchange__ = map.next_value()?;
                        }
                        GeneratedField::TsCollection => {
                            if ts_collection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsCollection"));
                            }
                            ts_collection__ = map.next_value()?;
                        }
                        GeneratedField::TsEvent => {
                            if ts_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsEvent"));
                            }
                            ts_event__ = map.next_value()?;
                        }
                        GeneratedField::UpdateType => {
                            if update_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateType"));
                            }
                            update_type__ = Some(map.next_value::<stream_market_update_response_v1::StreamMarketUpdateType>()? as i32);
                        }
                        GeneratedField::Snapshot => {
                            if snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            snapshot__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamMarketUpdateResponseV1 {
                    commodity: commodity__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    class: class__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    exchange: exchange__.unwrap_or_default(),
                    sequence_id: sequence_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    ts_exchange: ts_exchange__,
                    ts_collection: ts_collection__,
                    ts_event: ts_event__,
                    update_type: update_type__.unwrap_or_default(),
                    snapshot: snapshot__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamMarketUpdateResponseV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_market_update_response_v1::Snapshot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asks.is_empty() {
            len += 1;
        }
        if !self.bids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamMarketUpdateResponseV1.Snapshot", len)?;
        if !self.asks.is_empty() {
            struct_ser.serialize_field("asks", &self.asks)?;
        }
        if !self.bids.is_empty() {
            struct_ser.serialize_field("bids", &self.bids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stream_market_update_response_v1::Snapshot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asks",
            "bids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Asks,
            Bids,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "asks" => Ok(GeneratedField::Asks),
                            "bids" => Ok(GeneratedField::Bids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_market_update_response_v1::Snapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamMarketUpdateResponseV1.Snapshot")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<stream_market_update_response_v1::Snapshot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asks__ = None;
                let mut bids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Asks => {
                            if asks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asks"));
                            }
                            asks__ = Some(map.next_value()?);
                        }
                        GeneratedField::Bids => {
                            if bids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bids"));
                            }
                            bids__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(stream_market_update_response_v1::Snapshot {
                    asks: asks__.unwrap_or_default(),
                    bids: bids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamMarketUpdateResponseV1.Snapshot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_market_update_response_v1::snapshot::Order {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount != 0. {
            len += 1;
        }
        if self.price != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamMarketUpdateResponseV1.Snapshot.Order", len)?;
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.price != 0. {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stream_market_update_response_v1::snapshot::Order {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "amount",
            "price",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
            Price,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "amount" => Ok(GeneratedField::Amount),
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_market_update_response_v1::snapshot::Order;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamMarketUpdateResponseV1.Snapshot.Order")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<stream_market_update_response_v1::snapshot::Order, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                let mut price__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(stream_market_update_response_v1::snapshot::Order {
                    amount: amount__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamMarketUpdateResponseV1.Snapshot.Order", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_market_update_response_v1::StreamMarketUpdateType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::TradeBuy => "TRADE_BUY",
            Self::TradeSell => "TRADE_SELL",
            Self::TradeUnknown => "TRADE_UNKNOWN",
            Self::BestAsk => "BEST_ASK",
            Self::BestBid => "BEST_BID",
            Self::UpdatedAsk => "UPDATED_ASK",
            Self::UpdatedBid => "UPDATED_BID",
            Self::Snapshot => "SNAPSHOT",
            Self::ForceSnapshot => "FORCE_SNAPSHOT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for stream_market_update_response_v1::StreamMarketUpdateType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "TRADE_BUY",
            "TRADE_SELL",
            "TRADE_UNKNOWN",
            "BEST_ASK",
            "BEST_BID",
            "UPDATED_ASK",
            "UPDATED_BID",
            "SNAPSHOT",
            "FORCE_SNAPSHOT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_market_update_response_v1::StreamMarketUpdateType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_market_update_response_v1::StreamMarketUpdateType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_market_update_response_v1::StreamMarketUpdateType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::Unknown),
                    "TRADE_BUY" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::TradeBuy),
                    "TRADE_SELL" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::TradeSell),
                    "TRADE_UNKNOWN" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::TradeUnknown),
                    "BEST_ASK" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::BestAsk),
                    "BEST_BID" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::BestBid),
                    "UPDATED_ASK" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::UpdatedAsk),
                    "UPDATED_BID" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::UpdatedBid),
                    "SNAPSHOT" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::Snapshot),
                    "FORCE_SNAPSHOT" => Ok(stream_market_update_response_v1::StreamMarketUpdateType::ForceSnapshot),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamTradesRequestV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instrument_criteria.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamTradesRequestV1", len)?;
        if let Some(v) = self.instrument_criteria.as_ref() {
            struct_ser.serialize_field("instrumentCriteria", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamTradesRequestV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instrument_criteria",
            "instrumentCriteria",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstrumentCriteria,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instrumentCriteria" | "instrument_criteria" => Ok(GeneratedField::InstrumentCriteria),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamTradesRequestV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamTradesRequestV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamTradesRequestV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instrument_criteria__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstrumentCriteria => {
                            if instrument_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instrumentCriteria"));
                            }
                            instrument_criteria__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamTradesRequestV1 {
                    instrument_criteria: instrument_criteria__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamTradesRequestV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamTradesResponseV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.additional_properties.is_empty() {
            len += 1;
        }
        if self.amount != 0. {
            len += 1;
        }
        if !self.class.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.exchange.is_empty() {
            len += 1;
        }
        if !self.sequence_id.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if self.price != 0. {
            len += 1;
        }
        if self.side != 0 {
            len += 1;
        }
        if self.ts_exchange.is_some() {
            len += 1;
        }
        if self.ts_collection.is_some() {
            len += 1;
        }
        if self.ts_event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.StreamTradesResponseV1", len)?;
        if !self.additional_properties.is_empty() {
            struct_ser.serialize_field("additionalProperties", &self.additional_properties)?;
        }
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.class.is_empty() {
            struct_ser.serialize_field("class", &self.class)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.exchange.is_empty() {
            struct_ser.serialize_field("exchange", &self.exchange)?;
        }
        if !self.sequence_id.is_empty() {
            struct_ser.serialize_field("sequenceId", &self.sequence_id)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.price != 0. {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if self.side != 0 {
            let v = stream_trades_response_v1::TradeSide::from_i32(self.side)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.side)))?;
            struct_ser.serialize_field("side", &v)?;
        }
        if let Some(v) = self.ts_exchange.as_ref() {
            struct_ser.serialize_field("tsExchange", v)?;
        }
        if let Some(v) = self.ts_collection.as_ref() {
            struct_ser.serialize_field("tsCollection", v)?;
        }
        if let Some(v) = self.ts_event.as_ref() {
            struct_ser.serialize_field("tsEvent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamTradesResponseV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "additional_properties",
            "additionalProperties",
            "amount",
            "class",
            "code",
            "exchange",
            "sequence_id",
            "sequenceId",
            "id",
            "price",
            "side",
            "ts_exchange",
            "tsExchange",
            "ts_collection",
            "tsCollection",
            "ts_event",
            "tsEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AdditionalProperties,
            Amount,
            Class,
            Code,
            Exchange,
            SequenceId,
            Id,
            Price,
            Side,
            TsExchange,
            TsCollection,
            TsEvent,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "additionalProperties" | "additional_properties" => Ok(GeneratedField::AdditionalProperties),
                            "amount" => Ok(GeneratedField::Amount),
                            "class" => Ok(GeneratedField::Class),
                            "code" => Ok(GeneratedField::Code),
                            "exchange" => Ok(GeneratedField::Exchange),
                            "sequenceId" | "sequence_id" => Ok(GeneratedField::SequenceId),
                            "id" => Ok(GeneratedField::Id),
                            "price" => Ok(GeneratedField::Price),
                            "side" => Ok(GeneratedField::Side),
                            "tsExchange" | "ts_exchange" => Ok(GeneratedField::TsExchange),
                            "tsCollection" | "ts_collection" => Ok(GeneratedField::TsCollection),
                            "tsEvent" | "ts_event" => Ok(GeneratedField::TsEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamTradesResponseV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.StreamTradesResponseV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamTradesResponseV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut additional_properties__ = None;
                let mut amount__ = None;
                let mut class__ = None;
                let mut code__ = None;
                let mut exchange__ = None;
                let mut sequence_id__ = None;
                let mut id__ = None;
                let mut price__ = None;
                let mut side__ = None;
                let mut ts_exchange__ = None;
                let mut ts_collection__ = None;
                let mut ts_event__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AdditionalProperties => {
                            if additional_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalProperties"));
                            }
                            additional_properties__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Class => {
                            if class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            class__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value()?);
                        }
                        GeneratedField::Exchange => {
                            if exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exchange"));
                            }
                            exchange__ = Some(map.next_value()?);
                        }
                        GeneratedField::SequenceId => {
                            if sequence_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceId"));
                            }
                            sequence_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Side => {
                            if side__.is_some() {
                                return Err(serde::de::Error::duplicate_field("side"));
                            }
                            side__ = Some(map.next_value::<stream_trades_response_v1::TradeSide>()? as i32);
                        }
                        GeneratedField::TsExchange => {
                            if ts_exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsExchange"));
                            }
                            ts_exchange__ = map.next_value()?;
                        }
                        GeneratedField::TsCollection => {
                            if ts_collection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsCollection"));
                            }
                            ts_collection__ = map.next_value()?;
                        }
                        GeneratedField::TsEvent => {
                            if ts_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tsEvent"));
                            }
                            ts_event__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamTradesResponseV1 {
                    additional_properties: additional_properties__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    class: class__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    exchange: exchange__.unwrap_or_default(),
                    sequence_id: sequence_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    side: side__.unwrap_or_default(),
                    ts_exchange: ts_exchange__,
                    ts_collection: ts_collection__,
                    ts_event: ts_event__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.StreamTradesResponseV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_trades_response_v1::TradeSide {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for stream_trades_response_v1::TradeSide {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "BUY",
            "SELL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_trades_response_v1::TradeSide;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_trades_response_v1::TradeSide::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(stream_trades_response_v1::TradeSide::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(stream_trades_response_v1::TradeSide::Unknown),
                    "BUY" => Ok(stream_trades_response_v1::TradeSide::Buy),
                    "SELL" => Ok(stream_trades_response_v1::TradeSide::Sell),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TimestampValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("kaikosdk.TimestampValue", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimestampValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimestampValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct kaikosdk.TimestampValue")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TimestampValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(TimestampValue {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("kaikosdk.TimestampValue", FIELDS, GeneratedVisitor)
    }
}
