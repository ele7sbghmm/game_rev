@tool
extends Node3D

@onready var mi = $MeshInstance3D


func _init():
	print("workin' init")


func _ready() -> void:
	print("workin'")
	var p = "./collisionmodel1.hkt.PS3"
	var f = FileAccess.open(p, FileAccess.READ)
	if not f:
		print("not f")
		return
	f.big_endian = true
	
	var m = read_ea(f, 0x1540)
	mi.mesh = m


func _process(delta: float) -> void:
	pass


func _draw():
	pass


func read_ea(f, o):
	f.seek(o)
	
	f.seek(f.get_position() + 0x18)
	var vn = f.get_32()
	
	f.seek(f.get_position() + 8)
	var fn = f.get_32()
	
	f.seek(f.get_position() + 0xa8)
	
	var st = SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_TRIANGLES)
	
	for i in range(vn):
		st.add_vertex(Vector3(f.get_float(), f.get_float(), f.get_float()))
		f.get_float()
	for i in range(fn * 3):
		st.add_index(f.get_32())
	
	st.generate_normals()
	
	var mesh =  st.commit()
	var mat = StandardMaterial3D.new()
	
	mat.cull_mode = BaseMaterial3D.CULL_DISABLED
	mi.material_override = mat
	
	return mesh
