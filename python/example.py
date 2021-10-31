from discodisco import Server, Parameterizer, Analyzer

disco = Server()
# disco.start()
# disco.stop()

print(disco)


class CustomAnalyzer(Analyzer):
    pass


class CustomParameterizer(Parameterizer):
    pass


analyzer = CustomAnalyzer()
parameterizer = CustomParameterizer()

# descriptor = disco.add_analyzer(analyzer)
# descriptor = disco.add_parameterizer(parameterizer)
