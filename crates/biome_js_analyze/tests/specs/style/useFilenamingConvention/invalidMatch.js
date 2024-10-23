{
	"$schema": "../../../../../../packages/@biomejs/biome/configuration_schema.json",
	"linter": {
		"rules": {
			"style": {
				"useFilenamingConvention": {
					"level": "error",
					"options": {
						"match": "%(.+)[.](.+)",
						"filenameCases": ["camelCase"]
					}
				}
			}
		}
	}
}
