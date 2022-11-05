// class Source {
//     x = 0;
//     y = - 0;
//
//     // Just a list of names for now
//     creeps = [];
// }


var sourceDirector = {
    sourceIds : [],
    sourceCreepCounts: {},
    directedCreeps : [],

    fillSources() {
        // dirty hack
        for (var room in Game.rooms) {
            this.sourceIds = Game.rooms[room]
                .find(FIND_SOURCES)
                .map(source => source.id);
        }
    },

    chooseSource(creep) {
        if(this.directedCreeps.includes(creep.name)) {
            // This shouldn't happen ... but y'know. Just in case.
            return this.directedCreeps[creep.name];
        }

        this.sourceIds.sort((a, b) => this.sourceCreepCounts[a] > this.sourceCreepCounts[b])
        let source = this.sourceIds[0];

        console.log("Giving " + creep.name + " source " + source);


        creep.memory.source = source;
        this.directedCreeps.push(creep.name);
    },

    cleanUpCreep(name) {
        let index = this.directedCreeps.indexOf(name);

        delete this.directedCreeps[index];
    }
}

module.exports = sourceDirector;
