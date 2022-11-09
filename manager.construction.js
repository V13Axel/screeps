module.exports = {
    constructRoads(room) {
        if(Game.flags[room.name + 'RoadStartFlag'] && Game.flags[room.name + 'RoadEndFlag']) {
            let path = room.findPath(
                Game.flags[room.name + 'RoadStartFlag'].pos,
                Game.flags[room.name + 'RoadEndFlag'].pos,
                {
                    ignoreCreeps: true,
                    ignoreRoads: true,
                }
            )
            this.createRoadConstructionSitesAlongPath(room, path);

            Game.flags[room.name + 'RoadStartFlag'].remove();
            Game.flags[room.name + 'RoadEndFlag'].remove();
            return;
        }

        if(room.memory.hasRoads) {
            return;
        }

        let neededRoads = {
            'controller': [room.controller.pos],
            'sources': room.find(FIND_SOURCES),
            'extensions': room.find(FIND_STRUCTURES).filter(structure => structure.structureType == STRUCTURE_EXTENSION),
        };

        let spawn = room.find(FIND_MY_SPAWNS)[0];

        for (var [type, source] of Object.entries(neededRoads)) {
            source.forEach(dest => {
                let path = spawn.pos.findPathTo(dest);

                this.createRoadConstructionSitesAlongPath(room, path);
            })
        }

        room.memory.hasRoads = true;
    },

    createRoadConstructionSitesAlongPath(room, path) {
        path.forEach(pos => {
            // room.visual.circle(pos.x, pos.y);
            let location = room.getPositionAt(pos.x, pos.y);

            if(room.lookAt(location).length > 1) {
                return;
            }

            location.createConstructionSite(
                STRUCTURE_ROAD
            );
        });
    }
}
