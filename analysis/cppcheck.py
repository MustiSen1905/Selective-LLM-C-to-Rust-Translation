import xml.etree.ElementTree as ET
import json
import os
import subprocess


def run_cppcheck(source_path, xml_file):
    """
    Führt cppcheck aus und erzeugt einen XML-Report
    """
    cmd = [
        "cppcheck",
        "--enable=warning,performance",
        "--std=c11",
        "--xml",
        "--xml-version=2",
        source_path
    ]

    print("Führe cppcheck aus...")

    with open(xml_file, "w", encoding="utf-8") as f:
        subprocess.run(
            cmd,
            stderr=f,          # cppcheck schreibt XML auf stderr!
            stdout=subprocess.DEVNULL,
            check=False
        )

    print(f"CPPCheck XML erstellt: {xml_file}")


def create_empty_xml(xml_file):
    root = ET.Element("results")
    tree = ET.ElementTree(root)
    tree.write(xml_file, encoding="utf-8", xml_declaration=True)
    print(f"Leere XML-Datei erstellt: {xml_file}")


def xml_to_json(xml_file, json_file):
    errors_list = []

    if not os.path.exists(xml_file):
        create_empty_xml(xml_file)

    tree = ET.parse(xml_file)
    root = tree.getroot()

    for error in root.findall(".//error"):
        error_dict = {
            "id": error.attrib.get("id"),
            "severity": error.attrib.get("severity"),
            "msg": error.attrib.get("msg"),
            "locations": []
        }

        for loc in error.findall("location"):
            loc_dict = {
                "file": loc.attrib.get("file"),
                "line": int(loc.attrib.get("line", 0)),
                "column": int(loc.attrib.get("column", 0)),
                "info": loc.attrib.get("info", "")
            }
            error_dict["locations"].append(loc_dict)

        errors_list.append(error_dict)

    with open(json_file, "w", encoding="utf-8") as f:
        json.dump(errors_list, f, indent=4, ensure_ascii=False)

    print(f"JSON-Report erstellt: {json_file}")


def main(source_path, xml_file, json_file):
    # cppcheck ausführen
    run_cppcheck(source_path, xml_file)

    # XML → JSON
    xml_to_json(xml_file, json_file)


if __name__ == "__main__":
    main(
        source_path="src",                 # Ordner oder Datei
        xml_file="cppcheck_report.xml",
        json_file="cppcheck_report.json"
    )
