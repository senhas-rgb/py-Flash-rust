def fileRead(fileName):
    with open(fileName, 'r') as file:
        return file.read()

def subjectParse(data):
    availableSubjects = []
    import json
    parsed = json.loads(data)
    for subjects in parsed["subjects"]:
        availableSubjects.append(subjects["name"])
    return availableSubjects

if __name__ == "__main__":
    data = fileRead('cards.json')
    subjects = subjectParse(data)
    print(subjects)