.PHONY: setup download_salad_cloud_openapi

setup:
	npm install @openapitools/openapi-generator-cli -g
	openapi-generator-cli version
	cargo install sd

download_salad_cloud_openapi: setup
	curl https://raw.githubusercontent.com/SaladTechnologies/salad-cloud-docs/refs/heads/main/api-specs/salad-cloud.yaml -o salad-cloud.yaml

generate_salad_cloud_openapi:
	openapi-generator-cli generate \
	  -i salad-cloud.yaml \
	  -g rust \
	  -o ./ \
	  --additional-properties=useChrono=false,packageName=saladcloud,packageVersion=0.1.0,deriveDefault=false \
	  --type-mappings=DateTime=time::OffsetDateTime,date=time::Date \
	  --import-mappings=time::OffsetDateTime=time::OffsetDateTime,time::Date=time::Date
	sd -F '#[derive(Clone, Default,' '#[derive(Clone,' **/*.rs
