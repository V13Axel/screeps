module.exports = {
    spawnGivenBudget(role, budget) {
        let parts = _.clone(role.definition);
        let total = 0;
        let budgetFits = {};

        for (let [part, definition] of Object.entries(role.partsBudgets)) {
            budgetFits[part] = true;
            total += (definition.cost + 50);
        }

        let totalLoops = 0;

        while(Object.entries(budgetFits).filter(item => item[1]).length > 0 && totalLoops < 5) {
            for (var [partName, definition] of Object.entries(role.partsBudgets)) {
                if(definition.limit <= parts.filter(part => part == partName).length) {
                    continue;
                }
                if(total + (definition.cost + 50) <= budget * definition.costModifier) {
                    total += (definition.cost + 50);
                    parts.push(partName);
                    parts.push(MOVE);
                    continue;
                } 

                budgetFits[partName] = false;
            }
            totalLoops++;
        }

        parts.sort();

        if(parts.length < 3) {
            return role.definition;
        }

        return parts; 
    },

    spawnIfNecessary(activeSpawn, creeps, desiredNumber, roleDetails, startedSpawning) {
        if(
            !activeSpawn.spawning &&
                !startedSpawning &&
                creeps.length < desiredNumber &&
                activeSpawn.room.energyAvailable >= 300
        ) {
            // let body = roleDetails.definition;
            let body = this.spawnGivenBudget(
                roleDetails,
                Math.max(activeSpawn.room.energyAvailable * .8, 300)
            );
            let name = roleDetails.name + Game.time;
            let memory = { memory: { role: roleDetails.name.toLowerCase() } };

            startedSpawning = true;

            let result = activeSpawn.spawnCreep(
                body,
                name,
                memory
            );
        } 

        return startedSpawning;
    }
}
