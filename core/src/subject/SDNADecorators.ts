import { PerspectiveProxy } from "../perspectives/PerspectiveProxy";
import { Subject } from "./Subject";
import { capitalize, propertyNameToSetterName, singularToPlural, stringifyObjectLiteral } from "./util";

export class PerspectiveAction {
    action: string
    source: string
    predicate: string
    target: string
}

export function addLink(source: string, predicate: string, target: string): PerspectiveAction {
    return {
        action: "addLink",
        source,
        predicate,
        target,
    };
}

export function hasLink(predicate: string): string {
    return `triple(this, "${predicate}", _)`
}

interface InstanceQueryParams {
    where?: object;
    condition?: string;
}

export function instanceQuery(options?: InstanceQueryParams) {
    return function <T>(target: T, key: keyof T, descriptor: PropertyDescriptor) {
        const originalMethod = descriptor.value;
        if(typeof originalMethod !== "function") {
            throw new Error("instanceQuery decorator can only be applied to methods");
        }

        descriptor.value = async function(perspective: PerspectiveProxy): Promise<T[]> {
            let instances: T[] = []
            //@ts-ignore
            let subjectClassName = target.name
            let query = `subject_class("${subjectClassName}", C), instance(C, Instance)`
            if(options && options.where) {
                for(let prop in options.where) {
                    let value = options.where[prop]
                    query += `, property_getter(C, Instance, "${prop}", "${value}")`
                }
            }

            if(options && options.condition) {
                query += ', ' + options.condition
            }

            let results = await perspective.infer(query)
            if(results == false) {
                return instances
            }
            if(typeof results == "string") {
                throw results
            }
            for(let result of results) {
                let instance = result.Instance
                let subject = new Subject(perspective, instance, subjectClassName)
                await subject.init()
                instances.push(subject as T)
            }

            return instances
        }
    };
}


interface PropertyOptions {
    through?: string;
    initial?: string,
    required?: boolean,
    writable?: boolean,
    resolveLanguage?: string;
    getter?: string;
    setter?: string;
    local?: boolean
}
export function subjectProperty(opts: PropertyOptions) {
    return function <T>(target: T, key: keyof T) {
        target["__properties"] = target["__properties"] || {};
        target["__properties"][key] = target["__properties"][key] || {};
        target["__properties"][key] = { ...target["__properties"][key], ...opts }

        if (opts.writable) {
            const value = key as string
            target[`set${capitalize(value)}`] = () => {}
        }

        Object.defineProperty(target, key, {configurable: true, writable: true});
    };
}

interface FlagOptions {
    through: string;
    value: string;
}
export function subjectFlag(opts: FlagOptions) {
    return function <T>(target: T, key: keyof T) {
        target["__properties"] = target["__properties"] || {};
        target["__properties"][key] = target["__properties"][key] || {};
        target["__properties"][key] = {
            ...target["__properties"][key],
            through: opts.through,
            required: true,
            initial: opts.value,
            flag: true
        }

        // @ts-ignore
        target[key] = opts.value;

        Object.defineProperty(target, key, {configurable: true, writable: true});
    };
}

interface WhereOptions {
    isInstance?: any
    condition?: string
}
interface CollectionOptions {
    through: string,
    where?: WhereOptions,
    local?: boolean
}

export function subjectCollection(opts: CollectionOptions) {
    return function <T>(target: T, key: keyof T) {
        target["__collections"] = target["__collections"] || {};
        target["__collections"][key] = opts;

        const value = key as string
        target[`add${capitalize(value)}`] = () => {}
        target[`remove${capitalize(value)}`] = () => {}
        target[`setCollection${capitalize(value)}`] = () => {}

        Object.defineProperty(target, key, {configurable: true, writable: true});
    };
}

export function makeRandomPrologAtom(length: number): string {
    let result = '';
    let characters = 'abcdefghijklmnopqrstuvwxyz';
    let charactersLength = characters.length;
    for (let i = 0; i < length; i++) {
       result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
 }

 interface SDNAClassOptions {
    name: string;
}

export function SDNAClass(opts: SDNAClassOptions) {
    return function (target: any) {
        target.prototype.className = opts.name;
        target.className = opts.name;

        target.generateSDNA = function() {
            let sdna = ""
            let subjectName = opts.name
            let obj = target.prototype;

            let uuid = makeRandomPrologAtom(8)

            sdna += `subject_class("${subjectName}", ${uuid}).\n`


            let classRemoverActions = []

            let constructorActions = []
            if(obj.subjectConstructor && obj.subjectConstructor.length) {
                constructorActions = constructorActions.concat(obj.subjectConstructor)
            }

            let instanceConditions = []
            if(obj.isSubjectInstance && obj.isSubjectInstance.length) {
                instanceConditions = instanceConditions.concat(obj.isSubjectInstance)
            }

            let propertiesCode = []
            let properties = obj.__properties || {}
            for(let property in properties) {
                let propertyCode = `property(${uuid}, "${property}").\n`

                let { through, initial, required, resolveLanguage, writable, flag, getter, setter, local } = properties[property]

                if(resolveLanguage) {
                    propertyCode += `property_resolve(${uuid}, "${property}").\n`
                    propertyCode += `property_resolve_language(${uuid}, "${property}", "${resolveLanguage}").\n`
                }

                if(getter) {
                    propertyCode += `property_getter(${uuid}, Base, "${property}", Value) :- ${getter}.\n`
                } else if(through) {
                    propertyCode += `property_getter(${uuid}, Base, "${property}", Value) :- triple(Base, "${through}", Value).\n`

                    if(required) {
                        if(flag) {
                            instanceConditions.push(`triple(Base, "${through}", "${initial}")`)
                        } else {
                            instanceConditions.push(`triple(Base, "${through}", _)`)
                        }
                    }
                }

                if(setter) {
                    propertyCode += `property_setter(${uuid}, "${property}", Actions) :- ${setter}.\n`
                } else if (writable) {
                    let setter = obj[propertyNameToSetterName(property)]
                    if(typeof setter === "function") {
                        let action = [{
                            action: "setSingleTarget",
                            source: "this",
                            predicate: through,
                            target: "value",
                            ...(local && { local: true })
                        }]
                        propertyCode += `property_setter(${uuid}, "${property}", '${stringifyObjectLiteral(action)}').\n`
                    }
                }

                propertiesCode.push(propertyCode)

                if(initial) {
                    constructorActions.push({
                        action: "addLink",
                        source: "this",
                        predicate: through,
                        target: initial,
                    })

                    classRemoverActions = [{
                        action: "removeLink",
                        source: "this",
                        predicate: through,
                        target: initial,
                    }]
                }
            }

            let collectionsCode = []
            let collections = obj.__collections || {}
            for(let collection in collections) {
                let collectionCode = `collection(${uuid}, "${collection}").\n`

                let { through, where, local} = collections[collection]

                if(through) {
                    if(where) {
                        if(!where.isInstance && !where.condition) {
                            throw "'where' needs one of 'isInstance' or 'condition'"
                        }

                        let conditions = []

                        if(where.isInstance) {
                            let otherClass
                            if(where.isInstance.name) {
                                otherClass = where.isInstance.name
                            } else {
                                otherClass = where.isInstance
                            }
                            conditions.push(`instance(OtherClass, Target), subject_class("${otherClass}", OtherClass)`)
                        }

                        if(where.condition) {
                            conditions.push(where.condition)
                        }

                        const conditionString = conditions.join(", ")

                        collectionCode += `collection_getter(${uuid}, Base, "${collection}", List) :- setof(Target, (triple(Base, "${through}", Target), ${conditionString}), List).\n`
                    } else {
                        collectionCode += `collection_getter(${uuid}, Base, "${collection}", List) :- findall(C, triple(Base, "${through}", C), List).\n`
                    }

                    let collectionAdderAction = [{
                        action: "addLink",
                        source: "this",
                        predicate: through,
                        target: "value",
                        ...(local && { local: true })
                    }]

                    let collectionRemoverAction = [{
                        action: "removeLink",
                        source: "this",
                        predicate: through,
                        target: "value",
                    }]

                    let collectionSetterAction = [{
                        action: "collectionSetter",
                        source: "this",
                        predicate: through,
                        target: "value",
                        ...(local && { local: true })
                    }]
                    collectionCode += `collection_adder(${uuid}, "${singularToPlural(collection)}", '${stringifyObjectLiteral(collectionAdderAction)}').\n`
                    collectionCode += `collection_remover(${uuid}, "${singularToPlural(collection)}", '${stringifyObjectLiteral(collectionRemoverAction)}').\n`
                    collectionCode += `collection_setter(${uuid}, "${singularToPlural(collection)}", '${stringifyObjectLiteral(collectionSetterAction)}').\n`
                }

                collectionsCode.push(collectionCode)
            }

            let subjectContructorJSONString = stringifyObjectLiteral(constructorActions)
            sdna += `constructor(${uuid}, '${subjectContructorJSONString}').\n`
            let instanceConditionProlog = instanceConditions.join(", ")
            sdna += `instance(${uuid}, Base) :- ${instanceConditionProlog}.\n`
            sdna += "\n"
            sdna += `destructor(${uuid}, '${stringifyObjectLiteral(classRemoverActions)}').\n`
            sdna += "\n"
            sdna += propertiesCode.join("\n")
            sdna += "\n"
            sdna += collectionsCode.join("\n")

            return sdna
        }

        Object.defineProperty(target, 'type', {configurable: true});
    }
}