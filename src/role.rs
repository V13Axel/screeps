enum Role {
    Harvester,
    Builder,
    Upgrader,
}

struct CreepRole {
    name: String,
    role: Role,
    definition: Vec<String>
}
