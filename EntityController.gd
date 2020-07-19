extends Node2D

signal on_entity_click(entity)


onready var entity_scene = preload("res://SyncedEntity.tscn")
onready var texture_loader = get_node("../TextureLoader")

var _registered_entities = {}

func entity_clicked(entity):
	emit_signal("on_entity_click", entity)

func _on_LogicController_created_entities(entities):
	var db = get_node("../../LogicController")
	for entity in entities: 
		var scene = entity_scene.instance()
		_registered_entities[entity] = scene
		scene.setup(db, texture_loader, entity)
		add_child(scene)
		scene.connect("on_entity_click", self, "entity_clicked")

func _on_LogicController_deleted_entities(entities: Array):
	for entity in entities: 
		if _registered_entities.has(entity):
			remove_child(entity)

func _on_Entity_on_entity_click(entity):
	pass
